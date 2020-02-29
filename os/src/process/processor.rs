use crate::interrupt::{disable_and_store, enable_and_wfi, restore};
use crate::process::pool::ThreadPool;
use crate::process::thread::{Thread, ThreadStatus};
use crate::process::{ExitCode, ThreadID};
use alloc::boxed::Box;
use core::cell::UnsafeCell;

// Processor Status
pub struct Status {
    pool: Box<ThreadPool>,
    idle: Box<Thread>,
    current: Option<(ThreadID, Box<Thread>)>,
}

// Why there is a 'UnsafeCell' wrapper? Rust makes is effort to ensure the safety of multi-threads-accessing
// We can simply add this wrapper to disable the check.
// Why not mutex? Because we can assert that only one can access this function.
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
                // Switch to the acquired one
                status.current = Some(thread);
                status
                    .idle
                    .switch_to(&mut *status.current.as_mut().unwrap().1);

                // Switch back
                let (id, thread) = status.current.take().unwrap();
                status.pool.retrieve(id, thread);
            } else {
                // Wait for next interrupt
                enable_and_wfi();

                // Is that other kernel thread can not receive interrupt? No! __trap_ret enables it!
                // Disable and handle the switch
                disable_and_store();
            }
        }
    }

    // Where could this function be executed?
    // I may say that not a specific thread but from interrupt
    pub fn tick(&self) {
        let status = self.status();
        if !status.current.is_none() {
            // One is running
            if status.pool.tick() {
                // We need a change
                let flags = disable_and_store();

                // Switch to idle for next scheduling
                status
                    .current
                    .as_mut()
                    .unwrap()
                    .1
                    .switch_to(&mut status.idle);

                // Restore interrupt
                restore(flags);
            }
        } // Else for continuing idle thread (back to idle from interrupt)
    }

    pub fn sleep(&self) {
        let status = self.status();
        if !status.current.is_none() {
            let flags = disable_and_store();
            let tid = status.current.as_mut().unwrap().0;
            let info = status.pool.threads[tid].as_mut().unwrap();
            info.status = ThreadStatus::Sleeping;
            status.current.as_mut().unwrap().1.switch_to(&mut *status.idle);
            restore(flags);
        }
    }

    pub fn wake_up(&self, id: ThreadID) {
        let status = self.status();
        status.pool.wake_up(id);
    }

    pub fn current_tid(&self) -> usize {
        self.status().current.as_mut().unwrap().0 as usize
    }

    pub fn exit(&self, _code: ExitCode) -> ! {
        // Disable interrupt
        disable_and_store();

        // Get id
        let status = self.status();
        let id = status.current.as_ref().unwrap().0;

        // Exit and switch to idle
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
