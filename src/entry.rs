global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
pub extern "C" fn kernel_entry() -> ! {
    extern "C" {
        fn _start();
        fn boot_stack_top();
    }
    println!("Rust OS minimal kernel");
    println!("-> _start addr    = 0x{:x}", _start as usize);
    println!("-> boot_stack_top = 0x{:x}", boot_stack_top as usize);

    panic!("** Kernel panic **");

//    loop {}
}