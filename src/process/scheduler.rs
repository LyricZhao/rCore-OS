use crate::process::ThreadID;
use alloc::vec::Vec;

pub trait Scheduler {
    fn push(&mut self, id: ThreadID);
    fn pop(&mut self) -> Option<ThreadID>;
    fn tick(&mut self) -> bool;
    fn exit(&mut self, id: ThreadID);
}

#[derive(Default)]
struct RoundRobinInfo {
    valid: bool,
    time: usize,
    prev: usize,
    next: usize,
}

pub struct RoundRobinScheduler {
    threads: Vec<RoundRobinInfo>,
    max_time: usize,
    current: usize,
}

impl RoundRobinScheduler {
    pub fn new(max_time: usize) -> Self {
        let mut instance = RoundRobinScheduler {
            threads: Vec::default(),
            max_time,
            current: 0,
        };
        instance.threads.push(RoundRobinInfo::default());
        instance
    }
}

impl Scheduler for RoundRobinScheduler {
    fn push(&mut self, id: ThreadID) {
        let id = id + 1;
        if id + 1 > self.threads.len() {
            self.threads.resize_with(id + 1, Default::default);
        }

        if self.threads[id].time == 0 {
            self.threads[id].time = self.max_time;
        }

        let prev = self.threads[0].prev;
        self.threads[id].valid = true;
        self.threads[prev].next = id;
        self.threads[id].prev = prev;
        self.threads[0].prev = id;
        self.threads[id].next = 0;
    }

    fn pop(&mut self) -> Option<ThreadID> {
        let next = self.threads[0].next;
        if next != 0 {
            let next = self.threads[next].next;
            let prev = self.threads[next].prev;
            self.threads[next].prev = prev;
            self.threads[prev].next = next;
            self.threads[next].prev = 0;
            self.threads[next].next = 0;
            self.threads[next].valid = false;
            self.current = next;
            Some(next - 1)
        } else {
            None
        }
    }

    fn tick(&mut self) -> bool {
        let id = self.current;
        if id != 0 {
            self.threads[id].time -= 1;
            return self.threads[id].time == 0;
        }
        return true;
    }

    fn exit(&mut self, id: ThreadID) {
        let id = id + 1;
        if self.current == id {
            self.current = 0;
        }
    }
}
