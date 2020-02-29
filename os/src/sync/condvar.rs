use spin::Mutex;
use alloc::collections::VecDeque;
use crate::process::{ThreadID, current_tid, sleep, wake_up};

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