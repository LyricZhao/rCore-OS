use crate::sbi;
use core::fmt::{self, Write};

struct StdOut;

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
    sbi::console_putchar(ch as u8 as usize);
}

pub fn puts(s: &str) {
    for ch in s.chars() {
        putchar(ch);
    }
}

#[allow(dead_code)]
pub fn getchar() -> char {
    let ch = sbi::console_getchar() as u8;
    match ch {
        255 => '\0',
        ch => ch as char,
    }
}

pub fn getchar_option() -> Option<char> {
    let ch = sbi::console_getchar() as isize;
    match ch {
        -1 => None,
        ch => Some(ch as u8 as char),
    }
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
