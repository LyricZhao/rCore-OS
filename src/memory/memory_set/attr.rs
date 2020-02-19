use crate::memory::paging::PageEntry;
use riscv::interrupt::enable;

pub struct MemoryAttr {
    user: bool,
    read_only: bool,
    executable: bool,
}

impl MemoryAttr {
    pub fn new() -> Self {
        MemoryAttr {
            user: false,
            read_only: false,
            executable: false,
        }
    }

    pub fn set_user(mut self) -> Self {
        self.user = true;
        self
    }

    pub fn set_read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    pub fn set_executable(mut self) -> Self {
        self.executable = true;
        self
    }

    pub fn apply(&self, entry: &mut PageEntry) {
        entry.set_present(true);
        entry.set_user(self.user);
        entry.set_writable(!self.read_only);
        entry.set_executable(self.executable);
    }
}
