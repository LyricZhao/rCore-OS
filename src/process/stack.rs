use crate::alloc::alloc::{alloc, dealloc, Layout};
use crate::consts::KERNEL_STACK_SIZE;

// The usize var will be bottom
pub struct KernelStack(usize);

impl KernelStack {
    pub fn new() -> Self {
        let bottom = unsafe {
            alloc(Layout::from_size_align(KERNEL_STACK_SIZE, KERNEL_STACK_SIZE).unwrap()) as usize
        };
        KernelStack(bottom)
    }

    pub fn new_empty() -> KernelStack {
        KernelStack(0)
    }

    pub fn top(&self) -> usize {
        self.0 + KERNEL_STACK_SIZE
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.0 as _,
                Layout::from_size_align(KERNEL_STACK_SIZE, KERNEL_STACK_SIZE).unwrap(),
            );
        }
    }
}
