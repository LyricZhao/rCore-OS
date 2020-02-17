global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
pub extern "C" fn kernel_entry() -> ! {
    loop {}
}