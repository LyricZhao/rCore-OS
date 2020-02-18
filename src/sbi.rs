#![allow(dead_code)]

// TODO: Change to enum later
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
fn sbi_call(which: usize, arg_0: usize, arg_1: usize, arg_2: usize) -> usize {
    let ret;
    unsafe {
        asm!("ecall" // Assembler operands
            : "={x10}" (ret) // Output
            : "{x10}" (arg_0), "{x11}" (arg_1), "{x12}" (arg_2), "{x17}" (which) // Input
            : "memory" // Clobbered register list
            : "volatile"); // Options
    }
    ret
}

pub fn console_putchar(ch: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, ch, 0, 0);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

pub fn set_timer(time: u64) { // Timer will fire at the time
    #[cfg(target_pointer_width = "32")]
    sbi_call(SBI_SET_TIMER, time as usize, (time >> 32) as usize, 0);
    #[cfg(target_pointer_width = "64")]
    sbi_call(SBI_SET_TIMER, time as usize, 0, 0);
}