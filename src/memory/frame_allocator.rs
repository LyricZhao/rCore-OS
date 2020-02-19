use crate::consts::MAX_PHYSICAL_PAGES;

// Physical frame allocator
pub struct LinearFrameAllocator {
    pub flags: [bool; MAX_PHYSICAL_PAGES],
    pub offset: usize,
    pub size: usize,
}

impl LinearFrameAllocator {
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
