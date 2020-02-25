use crate::memory::manager::attr::MemoryAttr;
use crate::memory::manager::handler::ByFrame;
use crate::memory::manager::Manager;
use xmas_elf::program::{Flags, SegmentData, Type};
use xmas_elf::ElfFile;

pub trait ElfExt {
    fn new_manager(&self) -> Manager;
}

// The format of a user program will be ELF
impl ElfExt for ElfFile<'_> {
    fn new_manager(&self) -> Manager {
        let mut manager = Manager::new();
        for area in self.program_iter() {
            print!("{}", area);
            if area.get_type() != Ok(Type::Load) {
                continue;
            }

            let vaddr = area.virtual_addr() as usize;
            let size = area.mem_size() as usize;
            println!("Mapping vaddr = {:#x}", vaddr);

            // Note the raw data need to be handled by the library
            // We have to copy the data from the address library provided
            let data = match area.get_data(self).unwrap() {
                SegmentData::Undefined(data) => data,
                _ => unreachable!(),
            };

            manager.push(
                vaddr,
                vaddr + size,
                area.flags().to_attr(),
                ByFrame::new(),
                Some((data.as_ptr() as usize, data.len())),
            );
            println!("Finish");
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
        if self.is_execute() {
            flags.set_executable()
        } else {
            flags
        }
    }
}
