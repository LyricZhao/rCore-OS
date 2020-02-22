use crate::interrupt::{disable_and_store, enable_and_wfi, restore};
use crate::process::pool::ThreadPool;
use crate::process::thread::Thread;
use crate::process::{ExitCode, ThreadID};
use alloc::boxed::Box;
use core::cell::UnsafeCell;

pub struct Status {
    pool: Box<ThreadPool>,
    idle: Box<Thread>,
    current: Option<(ThreadID, Box<Thread>)>,
}

pub struct Processor {
    status: UnsafeCell<Option<Status>>,
}

unsafe impl Sync for Processor {}

impl Processor {
    pub const fn new() -> Processor {
        Processor {
            status: UnsafeCell::new(None),
        }
    }

    pub fn initialize(&self, idle: Box<Thread>, pool: Box<ThreadPool>) {
        unsafe {
            *self.status.get() = Some(Status {
                pool,
                idle,
                current: None,
            });
        }
    }

    pub fn run(&self) {
        Thread::boot().switch_to(&mut self.status().idle);
    }

    pub fn add_thread(&self, thread: Box<Thread>) {
        self.status().pool.add(thread);
    }

    fn status(&self) -> &mut Status {
        unsafe { &mut *self.status.get() }.as_mut().unwrap()
    }

    pub fn idle_main(&self) -> ! {
        let status = self.status();
        disable_and_store();

        loop {
            if let Some(thread) = status.pool.acquire() {
                status.current = Some(thread);
                status
                    .idle
                    .switch_to(&mut *status.current.as_mut().unwrap().1);
                let (id, thread) = status.current.take().unwrap();
                status.pool.retrieve(id, thread);
            } else {
                enable_and_wfi();
                disable_and_store();
            }
        }
    }

    pub fn tick(&self) {
        let status = self.status();
        if !status.current.is_none() {
            if status.pool.tick() {
                let flags = disable_and_store();
                status
                    .current
                    .as_mut()
                    .unwrap()
                    .1
                    .switch_to(&mut status.idle);
                restore(flags);
            }
        }
    }

    pub fn exit(&self, _code: ExitCode) -> ! {
        disable_and_store();
        let status = self.status();
        let id = status.current.as_ref().unwrap().0;

        status.pool.exit(id);

        status
            .current
            .as_mut()
            .unwrap()
            .1
            .switch_to(&mut status.idle);

        loop {}
    }
}
