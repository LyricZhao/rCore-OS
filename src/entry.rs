use crate::consts::*;
use crate::memory::{alloc, dealloc};

global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
pub extern "C" fn kernel_entry() -> ! {
    extern "C" {
        fn end();
    }
    println!("Rust OS minimal kernel");

    let kernel_end_paddr = end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR;
    println!("-> kernel space: [{:#x}, {:#x})", KERNEL_BEGIN_PADDR, kernel_end_paddr);
    println!("-> free space: [{:#x}, {:#x})", kernel_end_paddr, PHYSICAL_MEMORY_END);

    // Interrupt initialization
    crate::interrupt::initialize();

    // Timer initialization
    crate::timer::initialize();

    // Memory initialization
    crate::memory::initialize((kernel_end_paddr >> 12) + 1, PHYSICAL_MEMORY_END >> 12);

    // Allocate test
    println!("-> Alloc {:x?}", alloc());
    let frame = alloc();
    println!("-> Alloc {:x?}", frame);
    println!("-> Alloc {:x?}", alloc());
    println!("-> Dealloc {:x?}", frame);
    dealloc(frame.unwrap());
    println!("-> Alloc {:x?}", alloc());
    println!("-> Alloc {:x?}", alloc());

    loop {}
}