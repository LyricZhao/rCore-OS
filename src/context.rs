use riscv::register::{scause::Scause, sstatus::Sstatus};

#[repr(C)] // C-like Memory layout (in order)
           // #[derive(Debug)]
pub struct TrapFrame {
    pub x: [usize; 32],   // General purpose registers
    pub sstatus: Sstatus, // Supervisor status register
    pub sepc: usize,      // Supervisor exception PC
    pub stval: usize,     // Supervisor trap value
    pub scause: Scause,   // Cause of the exception
}
