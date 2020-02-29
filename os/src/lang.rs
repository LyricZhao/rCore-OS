use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // ! means will not return
    println!("{}", info);
    loop {}
}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("System aborts.");
}
