use crate::process::scheduler::Scheduler;
use crate::process::thread::{Thread, ThreadInfo, ThreadStatus};
use crate::process::ThreadID;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct ThreadPool {
    threads: Vec<Option<ThreadInfo>>,
    scheduler: Box<dyn Scheduler>,
}

impl ThreadPool {
    pub fn new(size: usize, scheduler: Box<dyn Scheduler>) -> ThreadPool {
        ThreadPool {
            threads: {
                let mut vec = Vec::new();
                vec.resize_with(size, Default::default);
                vec
            },
            scheduler,
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
        self.threads[id] = Some(ThreadInfo {
            status: ThreadStatus::Ready,
            thread: Some(thread),
        });
        self.scheduler.push(id);
    }

    // Acquire one from the pool and run
    pub fn acquire(&mut self) -> Option<(ThreadID, Box<Thread>)> {
        if let Some(id) = self.scheduler.pop() {
            let mut info = self.threads[id].as_mut().unwrap();
            info.status = ThreadStatus::Running(id);
            // Once we use 'take', the info.thread will be 'None' after used
            Some((id, info.thread.take().unwrap()))
        } else {
            None
        }
    }

    // Running for a long time or exit
    pub fn retrieve(&mut self, id: ThreadID, thread: Box<Thread>) {
        // Exited
        if self.threads[id].is_none() {
            return;
        }

        let mut info = self.threads[id].as_mut().unwrap();
        info.thread = Some(thread);
        if let ThreadStatus::Running(_) = info.status {
            // Running -> Ready
            info.status = ThreadStatus::Ready;
            self.scheduler.push(id);
        }
    }

    // Check whether we need a switch when ticked
    pub fn tick(&mut self) -> bool {
        self.scheduler.tick()
    }

    pub fn exit(&mut self, id: ThreadID) {
        self.threads[id] = None;
        self.scheduler.exit(id);
    }
}
