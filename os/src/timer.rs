use crate::sbi::set_timer;
use riscv::register::{sie, time};

pub static mut TICKS: usize = 0;

static TIME_BASE: u64 = 100000;

pub fn initialize() {
    unsafe {
        TICKS = 0;
        sie::set_stimer();
    }

    set_next_event();
    println!("[kernel] Timer initialized.");
}

pub fn set_next_event() {
    set_timer(time::read() as u64 + TIME_BASE);
}
