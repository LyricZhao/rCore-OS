use riscv::register::{stvec, sscratch, sstatus};
use crate::context::TrapFrame;

global_asm!(include_str!("trap/trap.asm"));

pub fn initialize() {
    unsafe {
        extern "C" {
            fn __trap_entry();
        }
        // We use sscratch register to judge when (U or S) the trap happens
        sscratch::write(0);
        // Direct mode: jump to ebase directly when trapped
        stvec::write(__trap_entry as usize, stvec::TrapMode::Direct);
        sstatus::set_sie();
    }
    println!("Interrupt initialized.");
}

#[no_mangle]
fn trap_handler(frame: &mut TrapFrame) {
    println!("Got trap.");
    frame.sepc += 2;
}