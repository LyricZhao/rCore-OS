use crate::consts::PAGE_SIZE;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VirtualPageRange {
    // The address range is virtual
    start: usize,
    end: usize,
}

impl Iterator for VirtualPageRange {
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

impl VirtualPageRange {
    pub fn new(start: usize, end: usize) -> Self {
        VirtualPageRange {
            start: start / PAGE_SIZE,
            end: (end - 1) / PAGE_SIZE,
        }
    }
}
