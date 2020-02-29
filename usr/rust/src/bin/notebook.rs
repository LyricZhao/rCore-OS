#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::io::getchar;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;

#[no_mangle]
pub fn main() {
    println!("Welcome to notebook!");
    loop {
        let ch = getchar();
        match ch {
            // Left or right
            LF | CR => {
                print!("{}", LF as char);
                print!("{}", CR as char)
            }
            // Normal cases
            _ => print!("{}", ch as char),
        }
    }
}
