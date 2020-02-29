#![allow(dead_code)]

use crate::consts::*;
use crate::memory::frame_allocator::LinearFrameAllocator;
use crate::memory::manager::attr::MemoryAttr;
use crate::memory::manager::handler::Linear;
use crate::memory::manager::Manager;
use buddy_system_allocator::LockedHeap;
use riscv::addr::Frame;
use riscv::register::sstatus;
use spin::Mutex;

mod frame_allocator;
pub mod manager;

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
    unsafe {
        sstatus::set_sum();
    }
    FRAME_ALLOCATOR.lock().initialize(begin, end);
    unsafe {
        DYNAMIC_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }

    // Remapping of different areas in kernel code and generate a new page table
    // The remapped in this function will be only used in kernel threads
    // TODO: can the remap process be deleted in new user thread?
    kernel_remap();
    println!("[kernel] Memory initialized.");
}

pub fn kernel_remap() {
    let mut manager = Manager::new();
    extern "C" {
        fn boot_stack();
        fn boot_stack_top();
    }
    manager.push(
        boot_stack as usize,
        boot_stack_top as usize,
        MemoryAttr::new(),
        Linear::new(PHYSICAL_MEMORY_OFFSET),
        None,
    );
    manager.push(
        paddr_to_vaddr(0x0c00_2000),
        paddr_to_vaddr(0x0c00_3000),
        MemoryAttr::new(),
        Linear::new(PHYSICAL_MEMORY_OFFSET),
        None,
    );
    manager.push(
        paddr_to_vaddr(0x1000_0000),
        paddr_to_vaddr(0x1000_1000),
        MemoryAttr::new(),
        Linear::new(PHYSICAL_MEMORY_OFFSET),
        None,
    );
    unsafe {
        manager.activate();
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
