use crate::trap::frame::TrapFrame;
use core::mem::zeroed;
use riscv::register::sstatus;

// Why there is a repr(C)
// To ensure the compiler will not optimize the order so that the asm code could access
#[repr(C)]
pub struct Content {
    pub ra: usize,    // Return address
    satp: usize,      // Page table register
    s: [usize; 12],   // Callee saved
    frame: TrapFrame, // Trap frame, we use features of *trap* frame to initialize the thread
}

impl Content {
    fn new_kernel(entry: usize, kernel_stack_top: usize, satp: usize) -> Content {
        extern "C" {
            fn __trap_ret();
        };
        Content {
            ra: __trap_ret as usize, // Why __trap_ret? __trap_ret restores the register values (we can set them to zero as an initialization)
            satp,
            s: [0; 12],
            frame: {
                let mut frame: TrapFrame = unsafe { zeroed() };
                frame.x[2] = kernel_stack_top;
                frame.sepc = entry;
                frame.sstatus = sstatus::read();
                frame.sstatus.set_spp(sstatus::SPP::Supervisor); // return with S mode
                frame.sstatus.set_spie(true); // Same as below
                frame.sstatus.set_sie(false); // return with async interrupt enabled
                frame
            },
        }
    }

    unsafe fn push_at(self, stack_top: usize) -> Context {
        let ptr = (stack_top as *mut Content).sub(1); // sub means minus sizeof(Content)
        *ptr = self;
        Context {
            content_addr: ptr as usize,
        }
    }
}

// How could we describe a context?
// The *content* will be stored in the stack, we only have to take care of the address of the stack top
#[repr(C)]
pub struct Context {
    pub content_addr: usize,
}

impl Context {
    #[naked] // Do not use *prologue* and *epilogue*, because the asm has already done
    #[inline(never)] // Do not inline, because we're using call/ret to switch thread
    pub unsafe extern "C" fn switch(&mut self, _target: &mut Context) {
        asm!(include_str!("switch.asm") :::: "volatile");
    }

    pub fn null() -> Context {
        Context { content_addr: 0 }
    }
}

impl Context {
    // New kernel thread (S mode)
    pub unsafe fn new_kernel(entry: usize, kernel_stack_top: usize, satp: usize) -> Context {
        Content::new_kernel(entry, kernel_stack_top, satp).push_at(kernel_stack_top)
    }

    // We can use the trap frame (as an initialization) to pass arguments
    pub unsafe fn append_args(&self, args: [usize; 3]) {
        let content = &mut *(self.content_addr as *mut Content);
        content.frame.x[10] = args[0];
        content.frame.x[11] = args[1];
        content.frame.x[12] = args[2];
    }
}
