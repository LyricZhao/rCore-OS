use crate::memory::frame_alloc;
use crate::memory::memory_set::attr::MemoryAttr;
use crate::memory::paging::PageTable;
use alloc::boxed::Box;
use core::fmt::Debug;

pub trait MemoryHandler: Debug + 'static {
    fn box_clone(&self) -> Box<dyn MemoryHandler>;
    fn map(&self, page_table: &mut PageTable, vaddr: usize, attr: &MemoryAttr);
    fn unmap(&self, page_table: &mut PageTable, vaddr: usize);
}

pub struct Linear {
    offset: usize,
}

impl Linear {
    pub fn new(offset: usize) -> Self {
        Linear { offset }
    }
}

impl MemoryHandler for Linear {
    fn box_clone(&self) -> Box<dyn MemoryHandler> {
        Box::new(self.clone())
    }

    fn map(&self, page_table: &mut PageTable, vaddr: usize, attr: &MemoryAttr) {
        attr.apply(page_table.map(vaddr, vaddr - self.offset));
    }

    fn unmap(&self, page_table: &mut PageTable, vaddr: usize) {
        page_table.unmap(vaddr);
    }
}

pub struct ByFrame;

impl ByFrame {
    pub fn new() -> Self {
        ByFrame {}
    }
}

impl MemoryHandler for ByFrame {
    fn box_clone(&self) -> Box<dyn MemoryHandler> {
        Box::new(self.clone())
    }

    fn map(&self, page_table: &mut PageTable, vaddr: usize, attr: &MemoryAttr) {
        let frame = frame_alloc().unwrap();
        let paddr = frame.start_address().as_usize();
        attr.apply(page_table.map(vaddr, paddr));
    }

    fn unmap(&self, page_table: &mut PageTable, vaddr: usize) {
        page_table.unmap(vaddr);
    }
}
