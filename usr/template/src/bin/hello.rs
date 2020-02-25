#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate template;

#[no_mangle]
pub fn main() -> usize {
    for _ in 0..10 {
        println!("Hello, world! (from user mode binary)");
    }
    0
}
