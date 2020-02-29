enum Syscall {
    Write = 64,
    Exit = 93,
}

#[inline(always)]
fn sys_call(id: Syscall, arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> i64 {
    let id = id as usize;
    let mut ret: i64;
    unsafe {
        // TODO: learn Rust inline ASM sometime
        asm!(
            "ecall"
            : "={x10}"(ret)
            : "{x17}"(id), "{x10}"(arg0), "{x11}"(arg1), "{x12}"(arg2), "{x13}"(arg3)
            : "memory"
            : "volatile"
        );
    }
    ret
}

pub fn sys_write(ch: u8) -> i64 {
    sys_call(Syscall::Write, ch as usize, 0, 0, 0)
}

pub fn sys_exit(code: usize) -> ! {
    sys_call(Syscall::Exit, code, 0, 0, 0);
    loop {}
}