use crate::memory::manager::Manager;
use xmas_elf::ElfFile;
use xmas_elf::program::Type::Load;
use xmas_elf::program::{SegmentData, Flags};
use crate::memory::manager::handler::ByFrame;
use crate::memory::manager::attr::MemoryAttr;

trait ElfExt {
    fn new_manager(&self) -> Manager;
}

impl ElfExt for ElfFile<'_> {
    fn new_manager(&self) -> Manager {
        let mut manager = Manager::new();
        for area in self.program_iter() {
            if area.get_type() != Ok(Load) {
                continue;
            }

            let vaddr = area.virtual_addr() as usize;
            let size = area.mem_size() as usize;
            let data = match area.get_data(self).unwrap() {
                SegmentData::Undefined(data) => data,
                _ => unreachable!(),
            };

            manager.push(vaddr, vaddr + size, area.flags().to_attr(), ByFrame::new(), Some((data.as_ptr() as usize, data.len())));
        }
        manager
    }
}

trait ToMemoryAttr {
    fn to_attr(&self) -> MemoryAttr;
}

impl ToMemoryAttr for Flags {
    fn to_attr(&self) -> MemoryAttr {
        let flags = MemoryAttr::new().set_user();
        if self.is_execute() { flags.set_executable() } else { flags }
    }
}