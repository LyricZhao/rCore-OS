use crate::consts::{USER_STACK_OFFSET, USER_STACK_SIZE};
use crate::memory::manager::attr::MemoryAttr;
use crate::memory::manager::handler::ByFrame;
use crate::process::context::Context;
use crate::process::elf::ElfExt;
use crate::process::stack::KernelStack;
use crate::process::{ExitCode, ThreadID};
use alloc::boxed::Box;
use riscv::register::satp;
use xmas_elf::{header, ElfFile};

pub struct Thread {
    pub context: Context,
    pub stack: KernelStack,
}

impl Thread {
    // The function is implemented by switching the context
    pub fn switch_to(&mut self, target: &mut Thread) {
        unsafe {
            self.context.switch(&mut target.context);
        }
    }

    // New kernel thread (S mode)
    pub fn new_kernel(entry: usize) -> Box<Thread> {
        unsafe {
            let stack = KernelStack::new();
            Box::new(Thread {
                context: Context::new_kernel(entry, stack.top(), satp::read().bits()),
                stack,
            })
        }
    }

    pub fn new_user(data: &[u8]) -> Box<Thread> {
        let elf = ElfFile::new(data).unwrap();

        match elf.header.pt2.type_().as_type() {
            header::Type::Executable => { }
            header::Type::SharedObject => {
                panic!("Shared object is not supported.");
            }
            _ => {
                panic!("Unsupported ELF type.");
            }
        }

        let entry = elf.header.pt2.entry_point() as usize;
        // The manager will add other areas into it
        let mut manager = elf.new_manager();

        let user_stack = {
            // User stack will be in a fixed space of kernel
            let (bottom, top) = (USER_STACK_OFFSET, USER_STACK_OFFSET + USER_STACK_SIZE);
            manager.push(
                bottom,
                top,
                MemoryAttr::new().set_user(),
                ByFrame::new(),
                None,
            );
            top
        };

        let kernel_stack = KernelStack::new();
        Box::new(Thread {
            context: unsafe {
                Context::new_user(entry, user_stack, kernel_stack.top(), manager.token())
            },
            stack: kernel_stack,
        })
    }

    pub fn boot() -> Box<Thread> {
        Box::new(Thread {
            context: Context::null(),
            stack: KernelStack::new_empty(),
        })
    }

    pub fn append_args(&self, args: [usize; 3]) {
        unsafe {
            self.context.append_args(args);
        }
    }
}

// TODO: fill up the info
#[derive(Clone)]
#[allow(dead_code)]
pub enum ThreadStatus {
    Ready,
    Running(ThreadID),
    Sleeping,
    Exited(ExitCode),
}

pub struct ThreadInfo {
    pub status: ThreadStatus,
    pub thread: Option<Box<Thread>>,
}
