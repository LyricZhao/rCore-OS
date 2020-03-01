use crate::consts::*;

global_asm!(include_str!("boot/entry64.asm"));

// User program code
global_asm!(include_str!("link_user.S"));

#[no_mangle]
pub extern "C" fn kernel_entry() -> ! {
    extern "C" {
        fn end();
    }
    println!("[kernel] rCore-OS Kernel");

    let kernel_end_paddr = end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR;

    // Memory initialization (initialize using physical page range)
    crate::memory::initialize(
        (kernel_end_paddr / PAGE_SIZE) + 1,
        PHYSICAL_MEMORY_END / PAGE_SIZE,
    );

    // Interrupt initialization
    crate::interrupt::initialize();

    /*
    // For lab-1
    unsafe {
        asm!("mret"::::"volatile");
    }
    */

    // Thread initialization
    crate::process::initialize();

    // Timer initialization
    crate::timer::initialize();

    // Start threads
    crate::process::run();

    loop {}
}
