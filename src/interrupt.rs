use riscv::register::{
    scause::{Exception, Interrupt, Trap},
    sscratch, sstatus, stvec,
};

use crate::context::TrapFrame;
use crate::timer::{set_next_event, TICKS};

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
    match frame.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint_handler(&mut frame.sepc),
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer_handler(),
        _ => panic!("undefined trap."),
    }
}

fn breakpoint_handler(sepc: &mut usize) {
    println!("breakpoint at 0x{:x}", sepc);
    *sepc += 2; // continue program
}

fn page_fault(frame: &mut TrapFrame) {
    println!(
        "{:?} vaddr = {:#x} instruction = {:#x}",
        frame.scause.cause(),
        frame.stval,
        frame.sepc
    );
    panic!("Page fault");
}

fn supervisor_timer_handler() {
    set_next_event();
    unsafe {
        TICKS += 1;
        if TICKS == 100 {
            TICKS = 0;
        }
    }
}
