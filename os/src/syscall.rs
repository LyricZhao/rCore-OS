use crate::process;

pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;

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
        _ => {
            panic!("Unknown syscall id {}", id);
        }
    }
}
