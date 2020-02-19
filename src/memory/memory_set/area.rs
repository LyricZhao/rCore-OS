use crate::consts::PAGE_SIZE;
use crate::memory::memory_set::attr::MemoryAttr;
use crate::memory::memory_set::handler::MemoryHandler;
use crate::memory::paging::{PageRange, PageTable};
use alloc::boxed::Box;

pub struct MemoryArea {
    start: usize,
    end: usize,
    handler: Box<dyn MemoryHandler>,
    attr: MemoryAttr,
}

impl MemoryArea {
    // Note (start, end) are the virtual address range
    pub fn new(
        start: usize,
        end: usize,
        handler: Box<dyn MemoryHandler>,
        attr: MemoryAttr,
    ) -> Self {
        MemoryArea {
            start,
            end,
            handler,
            attr,
        }
    }

    pub fn map(&self, page_table: &mut PageTable) {
        for page in PageRange::new(self.start, self.end) {
            self.handler.map(page_table, page, &self.attr);
        }
    }

    pub fn unmap(&self, page_table: &mut PageTable) {
        for page in PageRange::new(self.start, self.end) {
            self.handler.unmap(page_table, page);
        }
    }

    // The area is page size times, check whether overlapped with others
    pub fn is_overlap_with(&self, start: usize, end: usize) -> bool {
        let p1 = self.start / PAGE_SIZE;
        let p2 = (self.end - 1) / PAGE_SIZE + 1;
        let p3 = start / PAGE_SIZE;
        let p4 = (end - 1) / PAGE_SIZE + 1;
        !((p1 >= p4) || (p2 <= p3))
    }
}
