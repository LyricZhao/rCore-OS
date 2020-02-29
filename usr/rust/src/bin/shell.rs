#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;

use alloc::string::String;
use user::io::getchar;
use user::syscall::sys_exec;

#[no_mangle]
pub fn main() {
    println!("[ user ] rCore-OS User shell initialized.");
    let mut line: String = String::new();
    print!(">> ");
    loop {
        let ch = getchar();
        match ch {
            LF | CR => {
                println!();
                if !line.is_empty() {
                    sys_exec(line.as_ptr());
                    line.clear();
                }
                print!(">> ");
            }
            _ => {
                print!("{}", ch as char);
                line.push(ch as char);
            }
        }
    }
}
