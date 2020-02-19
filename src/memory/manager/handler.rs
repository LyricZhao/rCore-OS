use crate::memory::frame_alloc;
use crate::memory::manager::attr::MemoryAttr;
use crate::memory::manager::paging::table::PageTable;
use alloc::boxed::Box;

// TODO: why there is a Debug and 'static
// Memory handler is more likely a wrapper (for areas) of page tables (for page).
pub trait Handler: 'static {
    // Grammar 'dyn' is used to solve the ambiguity, MemoryHandler is a structure or trait (yes) ?
    fn box_clone(&self) -> Box<dyn Handler>;

    fn unmap(&self, page_table: &mut PageTable, vaddr: usize) {
        page_table.unmap(vaddr);
    }

    // The only difference between handlers
    fn map(&self, page_table: &mut PageTable, vaddr: usize, attr: &MemoryAttr);
}

impl Clone for Box<dyn Handler> {
    fn clone(&self) -> Box<dyn Handler> {
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

impl Handler for Linear {
    fn box_clone(&self) -> Box<dyn Handler> {
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

impl Handler for ByFrame {
    fn box_clone(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }

    fn map(&self, page_table: &mut PageTable, vaddr: usize, attr: &MemoryAttr) {
        let frame = frame_alloc().unwrap();
        let paddr = frame.start_address().as_usize();
        attr.apply(page_table.map(vaddr, paddr));
    }
}
