use crate::process;

pub const SYS_READ: usize = 63;
pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;
pub const SYS_EXEC: usize = 221;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    match id {
        SYS_WRITE => {
            // TODO: change to putchar
            print!("{}", args[0] as u8 as char);
            0
        }
        SYS_EXIT => {
            process::exit(args[0]);
        }
        SYS_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
        SYS_EXEC => sys_exec(args[0] as *const u8),
        _ => {
            panic!("Unknown syscall id {}", id);
        }
    }
}

fn sys_read(_fd: usize, base: *mut u8, _len: usize) -> isize {
    unsafe {
        *base = crate::fs::stdio::STDIN.pop() as u8;
    }
    1
}

pub unsafe fn from_cstr(s: *const u8) -> &'static str {
    use core::{slice, str};
    let len = (0usize..).find(|&i| *s.add(i) == 0).unwrap();
    str::from_utf8(slice::from_raw_parts(s, len)).unwrap()
}

fn sys_exec(path: *const u8) -> isize {
    let valid = process::execute(unsafe { from_cstr(path) }, Some(process::current_tid()));
    if valid {
        process::sleep();
    }
    0
}
