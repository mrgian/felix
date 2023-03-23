use core::arch::asm;
use core::fmt;

pub struct Printer {
    vga_pointer: u32,
}

//core lib needs to know how to print a string to implement its print formatted func
impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.prints(s);
        Ok(())
    }
}

impl Printer {
    //copy given char to memory pointed to vga_pointer
    pub fn printc(&mut self, c: char) {
        unsafe {
            asm!(
                "mov [{0}], {1}",
                in(reg) self.vga_pointer,
                in(reg_byte) c as u8,
            );
            self.vga_pointer += 2;
        }
    }

    //print a string by printing one char at the time
    pub fn prints(&mut self, s: &str) {
        for c in s.chars() {
            self.printc(c);
        }
    }

}

//macro for print!
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

//macro for println!
/*#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n", format_args!($($arg)*)));
}*/

//global printer
pub static mut PRINTER: Printer = Printer {
    vga_pointer: 0x000b8000
};

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        PRINTER.write_fmt(args).unwrap();
    }
}

/*fn print(message: &str) {
    unsafe {
        asm!("mov esi, {0:e}",
            "mov edi, 0x000b8000",
            "cld",
            "2:",
            "lodsb",
            "or al, al",
            "jz 1f",

            "mov [edi], al",
            "inc edi",
            "inc edi",

            "jmp 2b",
            "1:",
            in(reg) message.as_ptr());
    }
}*/