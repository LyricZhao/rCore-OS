use alloc::vec::Vec;
use crate::process::thread::{ThreadInfo, Thread};
use alloc::boxed::Box;
use crate::process::scheduler::Scheduler;
use crate::process::ThreadID;
use crate::process::status::Status;

pub struct ThreadPool {
    threads: Vec<Option<ThreadInfo>>,
    scheduler: Box<dyn Scheduler>
}

impl ThreadPool {
    pub fn new(size: usize, scheduler: Box<dyn Scheduler>) -> ThreadPool {
        ThreadPool {
            threads: Vec::with_capacity(size),
            scheduler
        }
    }

    fn alloc_id(&self) -> ThreadID {
        for (i, info) in self.threads.iter().enumerate() {
            if info.is_none() {
                return i;
            }
        }
        panic!("Thread alloc failed");
    }

    pub fn add(&mut self, thread: Box<Thread>) {
        let id = self.alloc_id();
        self.threads[id] = Some(ThreadInfo{
            status: Status::Ready,
            thread: Some(thread),
        })
        self.scheduler.push(id);
    }

    // TODO: what is Option.take?
    pub fn acquire(&mut self) -> Option<(ThreadID, Box<Thread>)> {
        if let Some(id) = self.scheduler.pop() {
            let mut info = self.threads[id].as_mut().unwrap();
            info.status = Status::Running(id);
            Some((id, info.thread.take().unwrap()))
        } else {
            None
        }
    }

    pub fn retrieve(&mut self, id: ThreadID, thread: Box<Thread>) {
        if self.threads[id].is_none() {
            return;
        }
        let mut info = self.threads[id].as_mut().unwrap();
        info.thread = Some(thread);
        if let Status::Running(_) = info.status {
            info.status = Status::Ready;
            self.scheduler.push(id);
        }
    }

    pub fn tick(&mut self) -> bool {
        self.scheduler.tick()
    }

    pub fn exit(&mut self, id: ThreadID) {
        self.threads[id] = None;
        self.scheduler.exit(id);
    }
}