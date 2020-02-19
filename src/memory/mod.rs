#![allow(dead_code)]

use crate::consts::*;
use crate::memory::frame_allocator::LinearFrameAllocator;
use crate::memory::memory_set::attr::MemoryAttr;
use crate::memory::memory_set::handler::Linear;
use crate::memory::memory_set::MemorySet;
use buddy_system_allocator::LockedHeap;
use riscv::addr::Frame;
use spin::Mutex;

mod frame_allocator;
pub mod memory_set;
pub mod paging;

// Frame allocator
static FRAME_ALLOCATOR: Mutex<LinearFrameAllocator> = Mutex::new(LinearFrameAllocator {
    flags: [false; MAX_PHYSICAL_PAGES],
    offset: 0,
    size: 0,
});

// Dynamic allocator on heap
#[global_allocator]
static DYNAMIC_ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn initialize(begin: usize, end: usize) {
    FRAME_ALLOCATOR.lock().initialize(begin, end);
    unsafe {
        DYNAMIC_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
    kernel_remap();
    println!("Memory initialized.");
}

pub fn kernel_remap() {
    let mut memory_set = MemorySet::new();
    extern "C" {
        fn boot_stack();
        fn boot_stack_top();
    }
    memory_set.push(
        boot_stack as usize,
        boot_stack_top as usize,
        MemoryAttr::new(),
        Linear::new(PHYSICAL_MEMORY_OFFSET),
    );
    unsafe {
        memory_set.activate();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler panic.")
}

pub fn frame_alloc() -> Option<Frame> {
    Some(Frame::of_ppn(FRAME_ALLOCATOR.lock().alloc()))
}

pub fn frame_dealloc(frame: Frame) {
    FRAME_ALLOCATOR.lock().dealloc(frame.number());
}

pub fn paddr_to_vaddr(paddr: usize) -> usize {
    paddr + PHYSICAL_MEMORY_OFFSET
}
