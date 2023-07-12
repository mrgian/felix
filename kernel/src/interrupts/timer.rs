//TIMER INTERRUPT HANDLER
//Used to trigger the cpu scheduler and to context switch

use crate::drivers::pic::PICS;
use crate::multitasking::task::CPUState;
use crate::multitasking::task::TASK_MANAGER;
use core::arch::asm;

use crate::memory::paging::PAGING;
use crate::memory::paging::TABLES;

pub const TIMER_INT: u8 = 32;

const APP_TARGET: u32 = 0x00a0_0000;
const APP_SIZE: u32 = 0x0001_0000;

//TIMER IRQ
#[naked]
pub extern "C" fn timer() {
    unsafe {
        asm!(
            //disable interrupts
            "cli",
            //save registers
            "push ebp",
            "push edi",
            "push esi",
            "push edx",
            "push ecx",
            "push ebx",
            "push eax",
            //call c function with esp as argument
            "push esp",
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
            //re-enable interrupts
            "sti",
            //return irq
            "iretd",
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn timer_handler(esp: u32) -> u32 {
    //trigger scheduler and return the esp returned by scheduler
    unsafe {
        let new_esp: u32 = TASK_MANAGER.schedule(esp as *mut CPUState) as u32;

        let slot = TASK_MANAGER.get_current_slot();
        let target = APP_TARGET + (slot as u32 * APP_SIZE);

        //map table 8 (0x02000000) to the address where the executable is loaded 
        TABLES[8].set(target);
        PAGING.set_table(8, &TABLES[8]);

        PICS.end_interrupt(TIMER_INT);

        return new_esp;
    }
}
