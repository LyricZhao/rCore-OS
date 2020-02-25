use crate::consts::{PAGE_SIZE, PHYSICAL_MEMORY_END, PHYSICAL_MEMORY_OFFSET};
use crate::memory::manager::area::Area;
use crate::memory::manager::attr::MemoryAttr;
use crate::memory::manager::handler::{Handler, Linear};
use crate::memory::manager::paging::table::PageTable;
use crate::memory::paddr_to_vaddr;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub mod area;
pub mod attr;
pub mod handler;
pub mod paging;

pub struct Manager {
    areas: Vec<Area>,
    page_table: PageTable,
}

impl Manager {
    pub fn new() -> Self {
        let mut memory_set = Manager {
            areas: Vec::new(),
            page_table: PageTable::new(),
        };
        memory_set.initialize();
        memory_set
    }

    // Map kernel and physical memory
    pub fn initialize(&mut self) {
        extern "C" {
            fn stext();
            fn etext();
            fn srodata();
            fn erodata();
            fn sdata();
            fn edata();
            fn sbss();
            fn ebss();
            fn end();
        }

        let offset = PHYSICAL_MEMORY_OFFSET;
        // .text RX
        self.push(
            stext as usize,
            etext as usize,
            MemoryAttr::new().set_read_only().set_executable(),
            Linear::new(offset),
            None,
        );
        // .rodata R
        self.push(
            srodata as usize,
            erodata as usize,
            MemoryAttr::new().set_read_only(),
            Linear::new(offset),
            None,
        );
        // .data RW
        self.push(
            sdata as usize,
            edata as usize,
            MemoryAttr::new().set_read_only(),
            Linear::new(offset),
            None,
        );
        // .bss RW
        self.push(
            sbss as usize,
            ebss as usize,
            MemoryAttr::new(),
            Linear::new(offset),
            None,
        );
        // Physical memory RW
        self.push(
            (end as usize / PAGE_SIZE + 1) * PAGE_SIZE,
            paddr_to_vaddr(PHYSICAL_MEMORY_END),
            MemoryAttr::new(),
            Linear::new(offset),
            None,
        );
    }

    // Push a new area
    pub fn push(
        &mut self,
        start: usize,
        end: usize,
        attr: MemoryAttr,
        handler: impl Handler,
        data: Option<(usize, usize)>,
    ) {
        let area = Area::new(start, end, Box::new(handler), attr);
        area.map(&mut self.page_table);
        if let Some((src, length)) = data {
            area.page_copy(&mut self.page_table, src, length);
        }
        self.areas.push(area);
    }

    fn test_free_area(&self, start: usize, end: usize) -> bool {
        self.areas
            .iter()
            .find(|area| area.is_overlap_with(start, end))
            .is_none()
    }

    // Switch to current page table
    pub unsafe fn activate(&self) {
        self.page_table.activate();
    }
}
