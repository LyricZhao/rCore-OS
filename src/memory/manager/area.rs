use crate::consts::PAGE_SIZE;
use crate::memory::manager::attr::MemoryAttr;
use crate::memory::manager::handler::Handler;
use crate::memory::manager::paging::range::VirtualPageRange;
use crate::memory::manager::paging::table::PageTable;
use alloc::boxed::Box;

pub struct Area {
    start: usize,
    end: usize,
    handler: Box<dyn Handler>,
    attr: MemoryAttr,
}

impl Area {
    // Note (start, end) are the virtual address range
    pub fn new(start: usize, end: usize, handler: Box<dyn Handler>, attr: MemoryAttr) -> Self {
        Area {
            start,
            end,
            handler,
            attr,
        }
    }

    pub fn map(&self, page_table: &mut PageTable) {
        for page in VirtualPageRange::new(self.start, self.end) {
            self.handler.map(page_table, page, &self.attr);
        }
    }

    pub fn unmap(&self, page_table: &mut PageTable) {
        for page in VirtualPageRange::new(self.start, self.end) {
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

    pub fn page_copy(&self, page_table: &mut PageTable, src: usize, length: usize) {
        let mut length = length;
        let mut src = src;
        for page in VirtualPageRange::new(self.start, self.end) {
            self.handler.page_copy(page_table, page, src, if length < PAGE_SIZE { length } else { PAGE_SIZE });
            src += PAGE_SIZE;
            if length >= PAGE_SIZE {
                length -= PAGE_SIZE;
            }
        }
    }
}
