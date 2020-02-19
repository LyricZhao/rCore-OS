use crate::consts::*;
use buddy_system_allocator::LockedHeap;
use riscv::addr::Frame;

mod allocator;

pub fn initialize(begin: usize, end: usize) {
    allocator::ALLOCATOR.lock().initialize(begin, end);
    heap_initialize();
    println!("Memory initialized.");
}

#[global_allocator]
static DYNAMIC_ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

fn heap_initialize() {
    unsafe {
        DYNAMIC_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler panic.")
}

pub fn alloc() -> Option<Frame> {
    Some(Frame::of_ppn(allocator::ALLOCATOR.lock().alloc()))
}

pub fn dealloc(frame: Frame) {
    allocator::ALLOCATOR.lock().dealloc(frame.number());
}
