use crate::memory::{frame_alloc, frame_dealloc};
use riscv::addr::Frame;
use riscv::paging::{FrameAllocator, FrameDeallocator};

pub mod entry;
pub mod range;
pub mod table;

struct FrameAllocatorForPaging;

impl FrameAllocator for FrameAllocatorForPaging {
    fn alloc(&mut self) -> Option<Frame> {
        frame_alloc()
    }
}

impl FrameDeallocator for FrameAllocatorForPaging {
    fn dealloc(&mut self, frame: Frame) {
        frame_dealloc(frame)
    }
}
