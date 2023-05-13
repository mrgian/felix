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
    pub fn prints(&self, s: &str) {
        unsafe {
            let ptr = s.as_ptr();
            let len = s.len();

            asm!("push eax", "push ebx","push ecx", "int 0x80", "pop ecx", "pop ebx", "pop eax", in("eax") 0, in("ebx") ptr as u32, in("ecx") len as u32);
        }
    }
}

//macro for print!
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

//macro for println!
#[macro_export]
macro_rules! println {
    () => {
        unsafe {
            $crate::PRINTER.prints("\n");
        }
    };


    ($($arg:tt)*) => {
        $crate::print!("{}", format_args!($($arg)*));
        unsafe {
            $crate::PRINTER.prints("\n");
        }
    };
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        PRINTER.write_fmt(args).unwrap();
    }
}
