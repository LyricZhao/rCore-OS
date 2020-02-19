use crate::consts::*;
use crate::memory::{frame_alloc, frame_dealloc, paddr_to_vaddr};
use riscv::addr::{Frame, Page, PhysAddr, VirtAddr};
use riscv::asm::{sfence_vma, sfence_vma_all};
use riscv::paging::{
    FrameAllocator, FrameDeallocator, Mapper, PageTable as PageTableEntryArray, PageTableEntry,
    PageTableFlags as EF, Rv39PageTable,
};
use riscv::register::satp;

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

// PageEntry (页项) is different with PageTableEntry (页表项)
pub struct PageEntry {
    entry: &'static mut PageTableEntry,
    page: Page, // Virtual Page
}

impl PageEntry {
    // TLB refresh
    pub fn update(&mut self) {
        unsafe { sfence_vma(0, self.page.start_address().as_usize()) }
    }

    pub fn accessed(&self) -> bool {
        self.entry.flags().contains(EF::ACCESSED)
    }

    pub fn clear_accessed(&mut self) {
        self.entry.flags_mut().remove(EF::ACCESSED);
    }

    pub fn dirty(&self) -> bool {
        self.entry.flags().contains(EF::DIRTY)
    }

    pub fn clear_dirty(&mut self) {
        self.entry.flags_mut().remove(EF::DIRTY);
    }

    pub fn writable(&self) -> bool {
        self.entry.flags().contains(EF::WRITABLE)
    }

    pub fn set_writable(&mut self, value: bool) {
        self.entry.flags_mut().set(EF::WRITABLE, value);
    }

    pub fn present(&self) -> bool {
        self.entry.flags().contains(EF::VALID | EF::READABLE)
    }

    pub fn set_present(&mut self, value: bool) {
        self.entry.flags_mut().set(EF::VALID | EF::READABLE, value);
    }

    pub fn user(&self) -> bool {
        self.entry.flags().contains(EF::USER)
    }

    pub fn set_user(&mut self, value: bool) {
        self.entry.flags_mut().set(EF::USER, value);
    }

    pub fn executable(&self) -> bool {
        self.entry.flags().contains(EF::EXECUTABLE)
    }

    pub fn set_executable(&mut self, value: bool) {
        self.entry.flags_mut().set(EF::EXECUTABLE, value);
    }

    pub fn target(&self) -> usize {
        self.entry.addr().as_usize()
    }

    pub fn set_target(&mut self, target: usize) {
        let flags = self.entry.flags();
        let frame = Frame::of_addr(PhysAddr::new(target));
        self.entry.set(frame, flags);
    }
}

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
        // Note that the size of the space is fixed (2^9 entry for riscv64)
        let table = unsafe { &mut *(paddr_to_vaddr(paddr) as *mut PageTableEntryArray) };
        table.zero();

        // TODO: figure out how the table works
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

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct PageRange {
    // The address range is virtual
    start: usize,
    end: usize,
}

impl Iterator for PageRange {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.start < self.end {
            let page = self.start << 12;
            self.start += 1;
            Some(page)
        } else {
            None
        }
    }
}

impl PageRange {
    pub fn new(start: usize, end: usize) -> Self {
        PageRange {
            start: start / PAGE_SIZE,
            end: (end - 1) / PAGE_SIZE,
        }
    }
}
