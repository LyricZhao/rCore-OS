use crate::process::context::Context;
use crate::process::stack::KernelStack;
use alloc::boxed::Box;
use riscv::register::satp;

pub struct Thread {
    pub context: Context,
    pub stack: KernelStack,
}

impl Thread {
    pub fn switch_to(&mut self, target: &mut Thread) {
        unsafe {
            self.context.switch(&mut target.context);
        }
    }

    pub fn new_kernel(entry: usize) -> Box<Thread> {
        unsafe {
            let stack = KernelStack::new();
            Box::new(Thread {
                context: Context::new_kernel_thread(entry, stack.top(), satp::read().bits()),
                stack,
            })
        }
    }

    pub fn get_boot_thread() -> Box<Thread> {
        Box::new(Thread {
            context: Context::null(),
            stack: KernelStack::new_empty(),
        })
    }

    pub fn append_initial_arguments(&self, args: [usize; 3]) {
        unsafe {
            self.context.append_initial_args(args);
        }
    }
}
