use crate::consts::*;
use crate::memory::manager::paging::entry::PageEntry;
use crate::memory::manager::paging::FrameAllocatorForPaging;
use crate::memory::{frame_alloc, paddr_to_vaddr};
use riscv::addr::{Frame, Page, PhysAddr, VirtAddr};
use riscv::asm::sfence_vma_all;
use riscv::paging::{
    Mapper, PageTable as PageTableEntryArray, PageTableEntry, PageTableFlags as EF, Rv39PageTable,
};
use riscv::register::satp;

// Note the table could only map virtual page to physical ones, but not manage the segments of memory (like bss or ...)
pub struct PageTable {
    // Rv39PageTable is 3-level structure
    page_table: Rv39PageTable<'static>,
    // Root frame of the 3-level structure
    root: Frame,
    entry: Option<PageEntry>,
}

impl PageTable {
    pub fn new() -> Self {
        // Allocate for a new physical frame
        let frame = frame_alloc().unwrap();
        let paddr = frame.start_address().as_usize();

        // Use the frame space to initialize Rv39PageTable (compiler accesses via vaddr)
        // Note that the size of the space is fixed (2^9 entry for riscv64 because page size is 2^12 (8 bytes for each entry))
        let table = unsafe { &mut *(paddr_to_vaddr(paddr) as *mut PageTableEntryArray) };
        table.zero();

        PageTable {
            page_table: Rv39PageTable::new(table, PHYSICAL_MEMORY_OFFSET),
            root: frame,
            entry: None,
        }
    }

    // Map a virtual page to a physical one
    pub fn map(&mut self, vaddr: usize, paddr: usize) -> &mut PageEntry {
        // Default
        let flags = EF::VALID | EF::READABLE | EF::WRITABLE;
        let page = Page::of_addr(VirtAddr::new(vaddr));
        let frame = Frame::of_addr(PhysAddr::new(paddr));

        // Use the lib to map the page to the frame
        self.page_table
            .map_to(page, frame, flags, &mut FrameAllocatorForPaging)
            .unwrap()
            .flush();
        self.get_entry(vaddr).unwrap()
    }

    // Unmap a virtual page
    pub fn unmap(&mut self, vaddr: usize) {
        let page = Page::of_addr(VirtAddr::new(vaddr));
        let (_, flush) = self.page_table.unmap(page).unwrap();
        flush.flush();
    }

    // Get the mapping
    fn get_entry(&mut self, vaddr: usize) -> Option<&mut PageEntry> {
        let page = Page::of_addr(VirtAddr::new(vaddr));
        if let Ok(entry) = self.page_table.ref_entry(page.clone()) {
            let entry = unsafe { &mut *(entry as *mut PageTableEntry) };
            self.entry = Some(PageEntry { entry, page });
            Some(self.entry.as_mut().unwrap())
        } else {
            None
        }
    }

    // Get token (root.number is the physical page number)
    pub fn token(&self) -> usize {
        self.root.number() | (8 << 60)
    }

    // Change satp register for another page table (switching)
    unsafe fn set_token(token: usize) {
        asm!("csrw satp, $0" :: "r"(token) :: "volatile");
    }

    // Current token
    fn current_token() -> usize {
        satp::read().bits()
    }

    // TLB refresh
    fn flush_tlb() {
        unsafe {
            sfence_vma_all();
        }
    }

    // Activate self
    pub unsafe fn activate(&self) {
        let old_token = Self::current_token();
        let new_token = self.token();
        if new_token != old_token {
            Self::set_token(new_token);
            Self::flush_tlb();
        }
    }
}
