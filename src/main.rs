#![no_std]
#![no_main]

#![feature(asm)]
#![feature(global_asm)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // ! means will not return
    loop {}
}

global_asm!(include_str!("boot/entry64.asm"));

pub fn putchar(ch: u8) {
    let _ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        asm!("ecall"
             : "={x10}" (_ret)
             : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
             : "memory"
             : "volatile"
        );
    }
}

#[no_mangle] // name of the function will be remained
pub extern "C" fn kernel_main() -> ! {
    putchar(b'O');
    putchar(b'K');
    putchar(b'\n');
    loop {}
}