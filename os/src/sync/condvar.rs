use crate::process::{current_tid, sleep, wake_up, ThreadID};
use alloc::collections::VecDeque;
use spin::Mutex;

// Condvar for 'condition var'
#[derive(Default)]
pub struct Condvar {
    queue: Mutex<VecDeque<ThreadID>>,
}

impl Condvar {
    pub fn new() -> Self {
        Condvar::default()
    }

    // Wait till some condition
    pub fn wait(&self) {
        self.queue.lock().push_back(current_tid());
        sleep();
    }

    // The condition is satisfied
    // Pop the first thread
    pub fn notify(&self) {
        let id = self.queue.lock().pop_front();
        if let Some(id) = id {
            wake_up(id);
        }
    }
}
