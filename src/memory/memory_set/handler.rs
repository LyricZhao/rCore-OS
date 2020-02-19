use crate::memory::frame_alloc;
use crate::memory::memory_set::attr::MemoryAttr;
use crate::memory::paging::PageTable;
use alloc::boxed::Box;

// TODO: why there is a Debug and 'static
// Memory handler is more likely a wrapper (for areas) of page tables (for page).
pub trait MemoryHandler: 'static {
    // Grammar 'dyn' is used to solve the ambiguity, MemoryHandler is a structure or trait (yes) ?
    fn box_clone(&self) -> Box<dyn MemoryHandler>;

    fn unmap(&self, page_table: &mut PageTable, vaddr: usize) {
        page_table.unmap(vaddr);
    }

    // The only difference between handlers
    fn map(&self, page_table: &mut PageTable, vaddr: usize, attr: &MemoryAttr);
}

impl Clone for Box<dyn MemoryHandler> {
    fn clone(&self) -> Box<dyn MemoryHandler> {
        self.box_clone()
    }
}

#[derive(Clone)]
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
}

#[derive(Clone)]
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
}
