#![no_std]

use core::arch::asm;
use core::fmt;

pub struct Printer {}

pub static mut PRINTER: Printer = Printer {};

//core lib needs to know how to print a string to implement its print formatted func
impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.prints(s);
        Ok(())
    }
}

impl Printer {
    pub fn printc(&self, c: char) {
        unsafe {
            asm!("push eax", "push ebx", "int 0x80", "pop ebx", "pop eax", in("eax") 0, in("ebx") c as u32);
        }
    }

    pub fn prints(&self, s: &str) {
        for c in s.chars() {
            self.printc(c);
        }
    }

    pub fn new_line(&self) {
        self.printc('\n');
    }
}

//macro for print!
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

//macro for println!
/*#[macro_export]
macro_rules! println {
    () => {
        unsafe {
            $crate::PRINTER.new_line();
        }
    };


    ($($arg:tt)*) => {
        $crate::print!("{}", format_args!($($arg)*));
        unsafe {
            $crate::PRINTER.new_line();
        }
    };
}*/

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        PRINTER.write_fmt(args).unwrap();
    }
}
