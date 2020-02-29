use crate::process::{current_tid, sleep, wake_up, ThreadID};
use alloc::collections::VecDeque;
use spin::Mutex;

#[derive(Default)]
pub struct Condvar {
    queue: Mutex<VecDeque<ThreadID>>,
}

impl Condvar {
    pub fn new() -> Self {
        Condvar::default()
    }

    pub fn wait(&self) {
        self.queue.lock().push_back(current_tid());
        sleep();
    }

    pub fn notify(&self) {
        let id = self.queue.lock().pop_front();
        if let Some(id) = id {
            wake_up(id);
        }
    }
}
