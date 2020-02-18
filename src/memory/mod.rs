use riscv::addr::Frame;

mod allocator;

pub fn initialize(begin: usize, end: usize) {
    allocator::ALLOCATOR.lock().initialize(begin, end);
    println!("Memory initialized.");
}

pub fn alloc() -> Option<Frame> {
    Some(Frame::of_ppn(allocator::ALLOCATOR.lock().alloc()))
}

pub fn dealloc(frame: Frame) {
    allocator::ALLOCATOR.lock().dealloc(frame.number());
}