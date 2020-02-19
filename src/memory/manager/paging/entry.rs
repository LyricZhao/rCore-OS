use riscv::addr::{Frame, Page as VirtualPage, PhysAddr};
use riscv::asm::sfence_vma;
use riscv::paging::{PageTableEntry, PageTableFlags as EF};

// PageEntry (页项) is different with PageTableEntry (页表项)
pub struct PageEntry {
    pub entry: &'static mut PageTableEntry,
    pub page: VirtualPage,
}

impl PageEntry {
    // TLB refresh
    pub fn update(&mut self) {
        // asid is the process id (0 for kernel, I guess)
        unsafe { sfence_vma(0, self.page.start_address().as_usize()) }
    }

    // TODO: to deeply understand the meaning of the flags
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

    // Physical frame target (return as address)
    pub fn target(&self) -> usize {
        self.entry.addr().as_usize()
    }

    pub fn set_target(&mut self, target: usize) {
        let flags = self.entry.flags();
        let frame = Frame::of_addr(PhysAddr::new(target));
        self.entry.set(frame, flags);
    }
}
