use crate::memory::{frame_alloc, paddr_to_vaddr};
use crate::memory::manager::attr::MemoryAttr;
use crate::memory::manager::paging::table::PageTable;
use alloc::boxed::Box;
use crate::consts::PAGE_SIZE;

// Memory handler is more likely a wrapper (for virtual pages in memory area, setting up the mapping by different ways).
// The handler must ensure no overlapping
pub trait Handler: 'static {
    // Grammar 'dyn' is used to solve the ambiguity, MemoryHandler is a structure or trait (yes) ?
    fn box_clone(&self) -> Box<dyn Handler>;

    fn unmap(&self, page_table: &mut PageTable, vaddr: usize) {
        page_table.unmap(vaddr);
    }

    // The only difference between handlers
    fn map(&self, page_table: &mut PageTable, vaddr: usize, attr: &MemoryAttr);

    fn page_copy(&self, page_table: &mut PageTable, vaddr: usize, src: usize, length: usize);
}

impl Clone for Box<dyn Handler> {
    fn clone(&self) -> Box<dyn Handler> {
        self.box_clone()
    }
}

// Use offset (linear mapping)
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

    fn page_copy(&self, page_table: &mut PageTable, vaddr: usize, src: usize, length: usize) {
        let paddr = page_table.get_entry(vaddr)
            .unwrap()
            .entry
            .addr()
            .as_usize();
        assert_eq!(vaddr, paddr_to_vaddr(paddr));
        assert_eq!(vaddr, paddr + self.offset);
        unsafe {
            let dst = core::slice::from_raw_parts_mut(
                vaddr as *mut u8,
                PAGE_SIZE,
            );
            if length > 0 {
                let src = core::slice::from_raw_parts(
                    src as *const u8,
                    PAGE_SIZE,
                );
                for i in 0..length { dst[i] = src[i]; }
            }
            for i in length..PAGE_SIZE { dst[i] = 0; }
        }
    }
}

// Allocate new frame for area mapping
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

    fn page_copy(&self, page_table: &mut PageTable, vaddr: usize, src: usize, length: usize) {
        let paddr = page_table.get_entry(vaddr)
            .unwrap()
            .entry
            .addr()
            .as_usize();
        unsafe {
            let dst = core::slice::from_raw_parts_mut(
                paddr_to_vaddr(paddr) as *mut u8,
                PAGE_SIZE,
            );
            if length > 0 {
                let src = core::slice::from_raw_parts(
                    src as *const u8,
                    PAGE_SIZE,
                );
                for i in 0..length { dst[i] = src[i]; }
            }
            for i in length..PAGE_SIZE { dst[i] = 0; }
        }
    }
}
