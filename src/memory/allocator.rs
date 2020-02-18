use crate::consts::MAX_PHYSICAL_PAGES;
use spin::Mutex;

pub struct LinearAllocator {
    flags: [bool; MAX_PHYSICAL_PAGES],
    offset: usize,
    size: usize
}

pub static ALLOCATOR: Mutex<LinearAllocator> = Mutex::new(LinearAllocator {
    flags: [false; MAX_PHYSICAL_PAGES], offset: 0, size: 0
});

impl LinearAllocator {
    pub fn initialize(&mut self, l: usize, r: usize) {
        self.offset = l;
        self.size = r - l;
    }

    pub fn alloc(&mut self) -> usize {
        for index in 0..self.size {
            if !self.flags[index] {
                self.flags[index] = true;
                return index + self.offset;
            }
        }
        panic!("Physical memory depleted");
    }

    pub fn dealloc(&mut self, index: usize) {
        let index = index - self.offset;
        assert!(index < self.size);
        assert!(self.flags[index]);
        self.flags[index] = false;
    }
}