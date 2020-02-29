use riscv::register::{
    scause::{Exception, Interrupt, Trap},
    sie, sscratch, sstatus, stvec,
};

use crate::memory::paddr_to_vaddr;
use crate::process::tick;
use crate::timer::set_next_event;
use crate::trap::frame::TrapFrame;

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

        sie::set_sext();

        // Disabled by OpenSBI, open external interrupt and serial manually
        init_external_interrupt();
        enable_serial_interrupt();
    }
    println!("[kernel] Interrupt initialized.");
}

pub unsafe fn init_external_interrupt() {
    const SERIAL: u32 = 0xa;
    let hart0_s_mode_interrupt_enables: *mut u32 = paddr_to_vaddr(0x0c00_2080) as *mut u32;
    hart0_s_mode_interrupt_enables.write_volatile(1 << SERIAL);
}

pub unsafe fn enable_serial_interrupt() {
    let uart16550: *mut u8 = paddr_to_vaddr(0x10000000) as *mut u8;
    uart16550.add(4).write_volatile(0x0B);
    uart16550.add(1).write_volatile(0x01);
}

#[no_mangle]
fn trap_handler(frame: &mut TrapFrame) {
    match frame.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint_handler(&mut frame.sepc),
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer_handler(),
        Trap::Exception(Exception::InstructionPageFault) => page_fault(frame),
        Trap::Exception(Exception::LoadPageFault) => page_fault(frame),
        Trap::Exception(Exception::StorePageFault) => page_fault(frame),
        Trap::Exception(Exception::UserEnvCall) => syscall(frame),
        Trap::Interrupt(Interrupt::SupervisorExternal) => external_handler(),
        Trap::Exception(Exception::IllegalInstruction) => panic!("Illegal instruction."), // For lab-1
        _ => panic!("Undefined trap."),
    }
}

fn external_handler() {
    access_serial();
}

fn access_serial() {
    match super::io::getchar() {
        Some(ch) => {
            crate::fs::stdio::STDIN.push({
                if ch == '\r' {
                    '\n'
                } else {
                    ch
                }
            });
        }
        None => {}
    }
}

fn syscall(frame: &mut TrapFrame) {
    // Return address (skip ecall)
    frame.sepc += 4;
    let ret = crate::syscall::syscall(frame.x[17], [frame.x[10], frame.x[11], frame.x[12]]);
    frame.x[10] = ret as usize;
}

fn breakpoint_handler(sepc: &mut usize) {
    println!("breakpoint at 0x{:x}", sepc);
    *sepc += 2; // continue bin
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
    tick();
}

// Why we are using this?
// Idle status can not be interrupted (e.g. the scheduling process running)
#[inline(always)]
pub fn disable_and_store() -> usize {
    let sstatus: usize;
    unsafe {
        // Disable all the async interrupt and return the old one
        asm!("csrci sstatus, 1 << 1" : "=r"(sstatus) ::: "volatile");
    }
    sstatus
}

#[inline(always)]
pub fn restore(flags: usize) {
    unsafe {
        // Restore to the original one
        asm!("csrs sstatus, $0" :: "r"(flags) :: "volatile");
    }
}

#[inline(always)]
pub fn enable_and_wfi() {
    unsafe {
        // Enable interrupt and wait for the next
        asm!("csrsi sstatus, 1 << 1; wfi" :::: "volatile");
    }
}
