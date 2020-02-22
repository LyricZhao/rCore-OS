use crate::process::ThreadID;

pub trait Scheduler {
    fn push(&mut self, id: ThreadID);
    fn pop(&mut self) -> Option<ThreadID>;
    fn tick(&mut self) -> bool;
    fn exit(&mut self, id: ThreadID);
}