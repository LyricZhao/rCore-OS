#![no_std]

#![feature(asm)]
#![feature(global_asm)]

#[macro_use]
mod io;

mod entry;
mod lang;
mod sbi;