#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]

#[macro_use]
mod io;

mod consts;
mod entry;
mod fs;
mod interrupt;
mod lang;
mod memory;
mod process;
mod sbi;
mod syscall;
mod timer;
mod trap;

extern crate alloc;
