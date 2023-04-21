use crate::fat::FAT;
use crate::print::PRINTER;
use core::arch::asm;

use crate::tss::TaskStateSegment;

const APP_TARGET: u32 = 0x0030_0000;

//SHELL
//Warning! Mutable static here
//TODO: Implement a mutex to get safe access to this
pub static mut SHELL: Shell = Shell {
    buffer: [0 as char; 256],
    arg: [0 as char; 11],
    cursor: 0,
};

const PROMPT: &str = "felix> ";

pub struct Shell {
    buffer: [char; 256],
    arg: [char; 11],
    cursor: usize,
}

impl Shell {
    pub fn init(&mut self) {
        self.buffer = [0 as char; 256];
        self.cursor = 0;

        unsafe {
            PRINTER.set_colors(0xc, 0);
            print!("{}", PROMPT);

            PRINTER.reset_colors();
        }
    }

    pub fn add(&mut self, c: char) {
        self.buffer[self.cursor] = c;
        self.cursor += 1;

        print!("{}", c);
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.buffer[self.cursor] = 0 as char;
            self.cursor -= 1;

            unsafe {
                PRINTER.delete();
            }
        }
    }

    pub fn enter(&mut self) {
        unsafe {
            PRINTER.new_line();
        }

        self.interpret();
        self.init();
    }

    #[allow(unused_unsafe)]
    fn interpret(&mut self) {
        match self.buffer {
            //test command
            b if self.is_command("ping") => {
                println!("PONG!");
            }

            //list root directory
            b if self.is_command("ls") => unsafe {
                FAT.list_entries();
            },

            //display content of file
            b if self.is_command("cat") => unsafe {
                self.cat(&b);
            },

            //jump to specified program
            b if self.is_command("run") => unsafe {
                self.run(&b);
            },

            //help command
            b if self.is_command("help") => {
                println!("Available commands:\nls - lists root directory entries\ncat <file> - displays content of a file");
            }

            //empty, do nothing
            b if b[0] == '\0' => {}

            //unknown command
            _ => {
                println!("Unknown command!");
            }
        }
    }

    pub unsafe fn cat(&mut self, b: &[char]) {
        for i in 4..15 {
            self.arg[i - 4] = b[i];
        }

        let entry = FAT.search_file(&self.arg);

        if entry.name[0] != 0 {
            FAT.read_file(&entry);

            for c in FAT.buffer {
                if c != 0 {
                    print!("{}", c as char);
                }
            }
            println!();
        } else {
            println!("File not found!");
        }
    }

    pub unsafe fn run(&mut self, b: &[char]) {
        for i in 4..15 {
            self.arg[i - 4] = b[i];
        }

        let entry = FAT.search_file(&self.arg);
        if entry.name[0] != 0 {
            FAT.read_file_to_target(&entry, APP_TARGET as *mut u32);

            unsafe {
                let tss = TaskStateSegment::new(APP_TARGET);
                let ptr = &tss as *const TaskStateSegment;

                asm!("xchg bx, bx");

                asm!("cli");

                asm!("ltr [{}]", in(reg) ptr as u32);
            }
        } else {
            println!("Program not found!");
        }
    }

    pub fn is_command(&self, command: &str) -> bool {
        let mut i = 0;
        for c in command.chars() {
            if c != self.buffer[i as usize] {
                return false;
            }
            i += 1;
        }
        true
    }
}
