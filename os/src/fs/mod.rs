use lazy_static::*;
use rcore_fs::vfs::*;
use rcore_fs_sfs::SimpleFileSystem;
use alloc::{ sync::Arc, vec::Vec };

pub mod device;
pub mod stdio;

lazy_static! {
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        let device = {
            extern "C" {
                fn _user_img_start();
                fn _user_img_end();
            };
            let start = _user_img_start as usize;
            let end = _user_img_end as usize;
            Arc::new(unsafe { device::MemDisk::new(start, end) })
        };
        let sfs = SimpleFileSystem::open(device).unwrap();
        sfs.root_inode()
    };
}

pub trait INodeExt {
    fn read_as_vec(&self) -> Result<Vec<u8>>;
}

impl INodeExt for dyn INode {
    fn read_as_vec(&self) -> Result<Vec<u8>> {
        let size = self.metadata()?.size;
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        self.read_at(0, buf.as_mut_slice())?;
        Ok(buf)
    }
}

pub fn initialize() {
    println!("Available programs in rust/ are:");
    let mut id = 0;
    let dir = ROOT_INODE.lookup("rust").unwrap();
    while let Ok(name) = dir.get_entry(id) {
        id += 1;
        println!("  {}", name);
    }
}