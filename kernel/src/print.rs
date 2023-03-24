use core::arch::asm;
use core::fmt;

const WIDTH: u16 = 80;
const HEIGHT: u16 = 25;

const VGA_START: u32 = 0x000b8000;

pub struct Printer {
    x: u16,
    y: u16,
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
    pub fn printc(&mut self, c: char, foreground: u8, background: u8) {
        //calculate pointer from coords
        let pointer = VGA_START + ((self.y * WIDTH + self.x) * 2) as u32;

        unsafe {
            //move char byte to pointer
            asm!(
                "mov [{0}], {1}",
                in(reg) pointer,
                in(reg_byte) c as u8,
            );

            //calculate color byte and move it to pointer + 1
            let color = (background as u8) << 4 | (foreground as u8);
            asm!(
                "mov [{0}], {1}",
                in(reg) pointer + 1,
                in(reg_byte) color as u8,
            );

            //increment x coord
            self.x += 1;

            //if x coord overflow go to new line
            if self.x > WIDTH {
                self.x = 0;
                self.y += 1;
            }
        }
    }

    //print a string by printing one char at the time
    pub fn prints(&mut self, s: &str) {
        //set coords to current cursor position
        let cursor = self.get_cursor_position();
        self.x = cursor.0;
        self.y = cursor.1;

        for c in s.chars() {
            self.printc(c, 12, 0);
        }

        //set cursors position to new coords
        self.set_cursor_position();
    }

    //get cursor position directly talking to vga hardware
    pub fn get_cursor_position(&self) -> (u16, u16) {
        let mut index: u16 = 0;

        unsafe {
            asm!("out dx, al", in("dx") 0x3d4 as u16, in("al") 0x0f as u8);
            let mut a: u8 = 0;
            asm!("in al, dx", out("al") a, in("dx") 0x3d5);

            index |= a as u16;

            asm!("out dx, al", in("dx") 0x3d4 as u16, in("al") 0x0e as u8);
            let b: u8;
            asm!("in al, dx", out("al") b, in("dx") 0x3d5);

            index |= (b as u16) << 8;
        }

        let x: u16 = index % WIDTH;
        let y: u16 = index / WIDTH;

        (x, y)
    }

    //set cursor position directly talking to vga hardware
    pub fn set_cursor_position(&self) {
        let mut index: u16 = self.y * WIDTH + self.x;

        unsafe {
            asm!("out dx, al", in("dx") 0x3d4 as u16, in("al") 0x0f as u8);
            asm!("out dx, al", in("dx") 0x3d5 as u16, in("al") (index & 0xff) as u8);
            asm!("out dx, al", in("dx") 0x3d4 as u16, in("al") 0x0e as u8);
            asm!("out dx, al", in("dx") 0x3d5 as u16, in("al") ((index >> 8) & 0xff) as u8);
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
    () => {
        $crate::print::new_line();
    };


    ($($arg:tt)*) => {
        $crate::print!("{}", format_args!($($arg)*));
        $crate::print::new_line();
    };
}

//global printer
pub static mut PRINTER: Printer = Printer{x: 0, y: 0};

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        PRINTER.write_fmt(args).unwrap();
    }
}

pub fn new_line() {
    unsafe {
        PRINTER.x = 0;
        PRINTER.y += 1;

        PRINTER.set_cursor_position();
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
