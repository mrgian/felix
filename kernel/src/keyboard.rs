use crate::interrupts::pic::PICS;
use crate::shell::SHELL;
use core::arch::asm;

pub const KEYBOARD_INT: u8 = 33;
pub const KEYBAORD_CONTROLLER: u8 = 0x60;

pub static mut KEYBOARD: Keyboard = Keyboard { lshift: false };

pub struct Keyboard {
    lshift: bool,
}

//keyboard handler
#[naked]
pub extern "C" fn keyboard() {
    unsafe {
        asm!("call keyboard_handler", "iretd", options(noreturn));
    }
}

#[no_mangle]
pub extern "C" fn keyboard_handler() {
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

            _ => {}
        }
    }

    //print char
    let key = scancode_to_char(scancode);

    if key != '\0' {
        unsafe {
            SHELL.add(key);
        }
    }
}

fn scancode_to_char(scancode: u8) -> char {
    let chars = "1234567890qwertyuiopasdfghjklzxcvbnm".as_bytes();

    let diff;
    match scancode {
        0x02..=0x0b => diff = 2,
        0x10..=0x19 => diff = 6,
        0x1e..=0x26 => diff = 10,
        0x2c..=0x32 => diff = 15,
        _ => diff = 0,
    }

    let index = (scancode - diff) as usize;

    let mut key;

    if index < chars.len() {
        key = chars[index];

        unsafe {
            if KEYBOARD.lshift {
                key -= 0x20; //make char uppercase
            }
        }
    } else {
        key = 0x00;
    }

    //space
    if scancode == 0x39 {
        key = 0x20;
    }

    key as char
}
