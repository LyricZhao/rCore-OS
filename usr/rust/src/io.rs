use crate::syscall::{sys_read, sys_write};
use core::fmt::{self, Write};

struct StdOut;

pub const STDIN: usize = 0;

impl fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        puts(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    StdOut.write_fmt(args).unwrap();
}

pub fn putchar(ch: char) {
    sys_write(ch as u8);
}

pub fn puts(s: &str) {
    for ch in s.chars() {
        putchar(ch);
    }
}

pub fn getchar() -> u8 {
    let mut ch = 0u8;
    assert_eq!(sys_read(STDIN, &mut ch, 1), 1);
    ch
}

// TODO: learn Rust macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::io::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
