use crate::consts::*;

global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
pub extern "C" fn kernel_entry() -> ! {
    extern "C" {
        fn end();
    }
    println!("Rust OS minimal kernel");

    let kernel_end_paddr = end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR;
    println!(
        "-> kernel space: [{:#x}, {:#x})",
        KERNEL_BEGIN_PADDR, kernel_end_paddr
    );
    println!(
        "-> free space: [{:#x}, {:#x})",
        kernel_end_paddr, PHYSICAL_MEMORY_END
    );

    // Interrupt initialization
    crate::interrupt::initialize();

    // Timer initialization
    crate::timer::initialize();

    // Memory initialization (initialize using physical page range)
    crate::memory::initialize(
        (kernel_end_paddr / PAGE_SIZE) + 1,
        PHYSICAL_MEMORY_END / PAGE_SIZE,
    );

    // Thread test
    crate::process::initialize();

    loop {}
}
