use crate::trap::frame::TrapFrame;
use core::mem::zeroed;
use riscv::register::sstatus;

#[repr(C)]
pub struct Content {
    pub ra: usize,    // Return address
    satp: usize,      // Page table register
    s: [usize; 12],   // Callee saved
    frame: TrapFrame, // Trap frame
}

impl Content {
    fn new_kernel_thread(entry: usize, kernel_stack_top: usize, satp: usize) -> Content {
        extern "C" {
            fn __trap_ret();
        };
        Content {
            ra: __trap_ret as usize, // return and restore the status (from interrupt handling to kernel thread)
            satp,
            s: [0; 12],
            frame: {
                let mut frame: TrapFrame = unsafe { zeroed() };
                frame.x[2] = kernel_stack_top;
                frame.sepc = entry;
                frame.sstatus = sstatus::read();
                frame.sstatus.set_spp(sstatus::SPP::Supervisor); // return with S mode
                                                                 // return with async interrupt enabled
                frame.sstatus.set_spie(true);
                frame.sstatus.set_sie(false);
                frame
            },
        }
    }

    unsafe fn push_at(self, stack_top: usize) -> Context {
        let ptr = (stack_top as *mut Content).sub(1);
        *ptr = self;
        Context {
            content_addr: ptr as usize,
        }
    }
}

#[repr(C)]
pub struct Context {
    pub content_addr: usize,
}

impl Context {
    #[naked]
    #[inline(never)]
    pub unsafe extern "C" fn switch(&mut self, _target: &mut Context) {
        asm!(include_str!("switch.asm") :::: "volatile");
    }

    pub fn null() -> Context {
        Context { content_addr: 0 }
    }
}

impl Context {
    pub unsafe fn new_kernel_thread(entry: usize, kernel_stack_top: usize, satp: usize) -> Context {
        Content::new_kernel_thread(entry, kernel_stack_top, satp).push_at(kernel_stack_top)
    }

    pub unsafe fn append_initial_args(&self, args: [usize; 3]) {
        let content = &mut *(self.content_addr as *mut Content);
        content.frame.x[10] = args[0];
        content.frame.x[11] = args[1];
        content.frame.x[12] = args[2];
    }
}
