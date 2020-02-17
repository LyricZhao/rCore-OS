use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // ! means will not return
    loop {}
}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("OS aborts!");
}