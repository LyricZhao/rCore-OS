use crate::process::context::Context;
use crate::process::stack::KernelStack;
use alloc::boxed::Box;
use riscv::register::satp;

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
