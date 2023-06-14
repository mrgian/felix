use core::arch::asm;

//CPU EXCEPTIONS HANDLERS

//handle excpetion based on interrupt number
#[no_mangle]
pub extern "C" fn exception_handler(int: u32, eip: u32, cs: u32, eflags: u32) {
    match int {
        0x00 => {
            libfelix::println!("DIVISION ERROR!");
        }
        0x06 => {
            libfelix::println!("INVALID OPCODE!");
        }
        0x08 => {
            libfelix::println!("DOUBLE FAULT!");
        }
        0x0D => {
            libfelix::println!("GENERAL PROTECTION FAULT!");
        }
        0x0E => {
            libfelix::println!("PAGE FAULT!");
        }
        0xFF => {
            libfelix::println!("EXCEPTION!");
        }
        _ => {
            libfelix::println!("EXCEPTION!");
        }
    }
    libfelix::println!("EIP: {:X}, CS: {:X}, EFLAGS: {:b}", eip, cs, eflags);

    loop {}
}

#[naked]
pub extern "C" fn div_error() {
    unsafe {
        asm!(
            "push 0x00",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn invalid_opcode() {
    unsafe {
        asm!(
            "push 0x06",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn double_fault() {
    unsafe {
        asm!(
            "push 0x08",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn general_protection_fault() {
    unsafe {
        asm!(
            "push 0x0d",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn page_fault() {
    unsafe {
        asm!(
            "push 0x0e",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn generic_handler() {
    unsafe {
        asm!(
            "push 0xff",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn)
        );
    }
}
