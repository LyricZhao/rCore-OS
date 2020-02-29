use core::slice::from_raw_parts_mut;
use rcore_fs::dev::*;
use spin::RwLock;

// Use a read-write lock
// Threads can read at a same time, but only one can write
pub struct MemDisk(RwLock<&'static mut [u8]>);

impl MemDisk {
    pub unsafe fn new(begin: usize, end: usize) -> Self {
        MemDisk(RwLock::new(from_raw_parts_mut(
            begin as *mut u8,
            end - begin,
        )))
    }
}

impl Device for MemDisk {
    // Read from memory into buffer
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        let slice = self.0.read();
        let len = buf.len().min(slice.len() - offset);
        buf[..len].copy_from_slice(&slice[offset..(offset + len)]);
        Ok(len)
    }

    // Write buffer into memory
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        let mut slice = self.0.write();
        let len = buf.len().min(slice.len() - offset);
        slice[offset..offset + len].copy_from_slice(&buf[..len]);
        Ok(len)
    }

    fn sync(&self) -> Result<()> {
        Ok(())
    }
}
