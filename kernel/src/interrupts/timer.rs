use crate::drivers::pic::PICS;
use core::arch::asm;

pub const TIMER_INT: u8 = 32;

//timer handler
#[naked]
pub extern "C" fn timer() {
    unsafe {
        asm!(
            //save registers
            "push ebp",
            "push edi",
            "push esi",
            "push edx",
            "push ecx",
            "push ebx",
            "push eax",
            
            //give esp and int num to c func
            "push esp",
            "push 0x32",
            "call timer_handler",

            //set esp to return value of c func
            "mov esp, eax",

            //restore registers
            "pop eax",
            "pop ebx",
            "pop ecx",
            "pop edx",
            "pop esi",
            "pop edi",
            "pop ebp",

            //return interrupt
            "iretd",
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn timer_handler(int: u32, esp: u32) -> u32 {    
    print!(".");

    PICS.end_interrupt(TIMER_INT);

    return esp;
}
