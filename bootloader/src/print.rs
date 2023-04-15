use core::arch::asm;
use core::fmt;

//PRINTER
//Warning! Mutable static here
//TODO: Implement a mutex to get safe access to this
pub static mut PRINTER: Printer = Printer {};

pub struct Printer {}

//core lib needs to know how to print a string to implement its print formatted func
impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.prints(s);
        Ok(())
    }
}

impl Printer {
    //print only a char
    pub fn printc(&self, c: char) {
        unsafe {
            asm!(
                "int 0x10", //tell the bios to write content of al to screen
                in("al") c as u8,
                in("ah") 0x0e as u8,
                in("bx") 0 as u16,
            );
        }
    }

    //print a string by printing one char at the time
    pub fn prints(&self, s: &str) {
        for c in s.chars() {
            self.printc(c);
        }
    }

    //set bios video mode to clear the screen
    #[allow(dead_code)]
    pub fn clear() {
        unsafe {
            asm!(
                "int 0x10",
                in("ax") 0x0003 as u16,
            );
        }
    }
}

//macro for print!
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

//macro for println!
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        PRINTER.write_fmt(args).unwrap();
    }
}

//bios interrupt to print to the screen
/*pub fn print(message: &str) {
    unsafe {
        asm!("mov si, {0:x}", //move given string address to si
            "2:",
            "lodsb", //load a byte (next character) from si to al
            "or al, al", //bitwise or on al, if al is null set zf to true
            "jz 1f", //if zf is true (end of string) jump to end

            "mov ah, 0x0e",
            "mov bh, 0",
            "int 0x10", //tell the bios to write content of al to screen

            "jmp 2b", //start again
            "1:",
            in(reg) message.as_ptr());
    }
}*/
