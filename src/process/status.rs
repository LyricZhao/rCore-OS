use crate::process::{ThreadID, ExitCode};

#[derive(Clone)]
pub enum Status {
    Ready,
    Running(ThreadID),
    Sleeping,
    Exited(ExitCode)
}