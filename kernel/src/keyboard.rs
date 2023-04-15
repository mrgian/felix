use crate::interrupts::pic::PICS;
use crate::print::PRINTER;
use crate::shell::SHELL;
use core::arch::asm;

pub static mut KEYBOARD: Keyboard = Keyboard { lshift: false };

pub const KEYBOARD_INT: u8 = 33;
pub const KEYBAORD_CONTROLLER: u8 = 0x60;
pub const CHAR_COUNT: usize = 36;

pub struct Keyboard {
    lshift: bool,
}

//keyboard handler
#[naked]
pub extern "C" fn keyboard() {
    unsafe {
        //push charset to keyboard handler before calling
        asm!(
            "push 0x6d6e6276",
            "push 0x63787a6c",
            "push 0x6b6a6867",
            "push 0x66647361",
            "push 0x706f6975",
            "push 0x79747265",
            "push 0x77713039",
            "push 0x38373635",
            "push 0x34333231",
            "call keyboard_handler",
            "add esp, 36",
            "iretd",
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn keyboard_handler(charset: [u8; CHAR_COUNT]) {
    //read scancode from keyboard controller
    let scancode: u8;
    unsafe {
        asm!("in al, dx", out("al") scancode, in("dx") KEYBAORD_CONTROLLER as u16);
    }

    //notify pics end of interrupt
    PICS.end_interrupt(KEYBOARD_INT);

    unsafe {
        match scancode {
            //press left shift
            0x2a => {
                KEYBOARD.lshift = true;
                return;
            }

            //release left shift
            0xaa => {
                KEYBOARD.lshift = false;
                return;
            }

            //backspace
            0x0e => {
                SHELL.backspace();
                return;
            }

            //enter
            0x1c => {
                SHELL.enter();
                return;
            }

            //scroll
            0x48 => {
                PRINTER.scroll();
            }

            _ => {}
        }
    }

    //print char
    let key = scancode_to_char(scancode, charset);

    if key != '\0' {
        unsafe {
            SHELL.add(key);
        }
    }
}

fn scancode_to_char(scancode: u8, charset: [u8; CHAR_COUNT]) -> char {
    let diff;
    match scancode {
        0x02..=0x0b => diff = 2,
        0x10..=0x19 => diff = 6,
        0x1e..=0x26 => diff = 10,
        0x2c..=0x32 => diff = 15,
        _ => diff = 0,
    }

    let index = (scancode - diff) as usize;

    let mut key: char = '\0';

    if index < charset.len() {
        key = charset[index] as char;

        unsafe {
            if KEYBOARD.lshift {
                key = key.to_ascii_uppercase();
            }
        }
    }

    //space
    if scancode == 0x39 {
        key = 0x20 as char;
    }

    key
}
