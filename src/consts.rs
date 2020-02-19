pub const PHYSICAL_MEMORY_END: usize = 0x88000000;

pub const KERNEL_BEGIN_PADDR: usize = 0x80200000;
pub const KERNEL_BEGIN_VADDR: usize = 0xffffffffc0200000;

pub const MAX_PHYSICAL_MEMORY: usize = 0x8000000; // 128 MB
pub const MAX_PHYSICAL_PAGES: usize = MAX_PHYSICAL_MEMORY / PAGE_SIZE;

pub const KERNEL_HEAP_SIZE: usize = 0x800000; // 8 MB

pub const PHYSICAL_MEMORY_OFFSET: usize = 0xffffffff_40000000;

// Note that PAGE_SIZE is always (1 << 12) bytes in riscv64
pub const PAGE_SIZE: usize = 4096;
