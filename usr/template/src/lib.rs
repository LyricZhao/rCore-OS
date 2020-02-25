#![no_std]
#![feature(asm)]
#![feature(lang_items)]
#![feature(panic_info_message)]
#![feature(linkage)]

extern crate alloc;

#[macro_use]
pub mod io;

pub mod lang;
pub mod syscall;

use buddy_system_allocator::LockedHeap;

#[global_allocator]
static DYNAMIC_ALLOCATOR: LockedHeap = LockedHeap::empty();
