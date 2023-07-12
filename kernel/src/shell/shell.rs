//SHELL

use crate::filesystem::fat::FAT;
use crate::syscalls::print::PRINTER;
use crate::multitasking::task::TASK_MANAGER;

const APP_TARGET: u32 = 0x00a0_0000;
const APP_SIGNATURE: u32 = 0xB16B00B5;

const HELP: &'static str = "Available commands:
ls - lists root directory entries
cat <file> - displays content of a file
test <a,b,c> - runs a dummy task
run <file> - loads file as task and adds it to the task list
ps - lists running tasks
rt <id> - removes specified task";

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
    //init shell
    pub fn init(&mut self) {
        self.buffer = [0 as char; 256];
        self.cursor = 0;

        unsafe {
            PRINTER.set_colors(0xc, 0);
            libfelix::print!("{}", PROMPT);

            PRINTER.reset_colors();
        }
    }

    //adds new char to shell buffer
    pub fn add(&mut self, c: char) {
        self.buffer[self.cursor] = c;
        self.cursor += 1;

        libfelix::print!("{}", c);
    }

    //backspace, removes last char from buffer and updates cursor
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.buffer[self.cursor] = 0 as char;
            self.cursor -= 1;

            unsafe {
                PRINTER.delete();
            }
        }
    }

    //shell enter
    pub fn enter(&mut self) {
        unsafe {
            PRINTER.new_line();
        }

        self.interpret();
        self.init();
    }

    //command interpreter
    #[allow(unused_unsafe)]
    fn interpret(&mut self) {
        match self.buffer {
            //test command
            _b if self.is_command("ping") => {
                libfelix::println!("PONG!");
            },

            //list root directory
            _b if self.is_command("ls") => unsafe {
                FAT.list_entries();
            },

            //list running tasks
            _b if self.is_command("ps") => unsafe {
                TASK_MANAGER.list_tasks();
            },

            //remove runing task
            b if self.is_command("rt") => unsafe {
                if (b[3] as u8) < 0x30 {
                    libfelix::println!("No task id provided!");
                    return;
                }

                //convert first char of arg to id
                let id = ((b[3] as u8) - 0x30) as usize;

                TASK_MANAGER.remove_task(id);
                //TASK_MANAGER.remove_current_task();
            },

            //display content of file
            b if self.is_command("cat") => unsafe {
                self.cat(&b);
            },

            //jump to specified program
            b if self.is_command("run") => unsafe {
                self.run(&b);
            },

            //run test task
            b if self.is_command("test") => unsafe {
                let a = b[5];

                match a {
                    'a' => {
                        TASK_MANAGER.add_dummy_task_a();
                    }
                    'b' => {
                        TASK_MANAGER.add_dummy_task_b();
                    }
                    'c' => {
                        TASK_MANAGER.add_dummy_task_c();
                    }
                    _ => {
                        libfelix::println!("Specify test a, b, or c!");
                    }
                }
            },

            //help command
            _b if self.is_command("help") => {
                libfelix::println!("{}", HELP);
            }

            //empty, do nothing
            b if b[0] == '\0' => {}

            //unknown command
            _ => {
                libfelix::println!("Unknown command!");
            }
        }
    }

    //shows content of a file in ascii format
    pub unsafe fn cat(&mut self, b: &[char]) {
        for i in 4..15 {
            self.arg[i - 4] = b[i];
        }

        let entry = FAT.search_file(&self.arg);

        if entry.name[0] != 0 {
            FAT.read_file_to_buffer(entry);

            for c in FAT.buffer {
                if c != 0 {
                    libfelix::print!("{}", c as char);
                }
            }
            libfelix::println!();
        } else {
            libfelix::println!("File not found!");
        }
    }

    //loads an executable as a task
    pub unsafe fn run(&mut self, b: &[char]) {
        for i in 4..15 {
            self.arg[i - 4] = b[i];
        }

        let entry = FAT.search_file(&self.arg);
        if entry.name[0] != 0 {
            FAT.read_file_to_target(&entry, APP_TARGET as *mut u32);

            unsafe {
                let signature = *(APP_TARGET as *mut u32);

                if signature == APP_SIGNATURE {
                    TASK_MANAGER.run_app((APP_TARGET + 4) as u32);
                } else {
                    libfelix::println!("File is not a valid executable!");
                }
            }
        } else {
            libfelix::println!("Program not found!");
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

