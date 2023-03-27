use core::mem::size_of;
use core::arch::asm;

const IDT_ENTRIES: usize = 256;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset_low: u16,        //lower 16 bits of handler func address
    segment_selector: u16,  //segment selector of gdt entry
    reserved: u8,           //always zero
    flags: u8,              //entry flags
    offset_high: u16,       //higher 16 bits of handler func address
}

#[repr(C, packed)]
pub struct InterruptDescriptorTable {
    entries: [IdtEntry; IDT_ENTRIES],
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    size: u16,                                  //idt size                  
    offset: *const InterruptDescriptorTable,    //pointer to idt
}

impl InterruptDescriptorTable {
    pub fn new(handler: u32) -> Self {
        Self {
            entries: [IdtEntry::new(handler); IDT_ENTRIES],
        }
    }

    //load idt using lidt instruction
    pub fn load(&self) {
        let descriptor = IdtDescriptor {
            size: (IDT_ENTRIES * size_of::<IdtEntry>() - 1) as u16, //calculate size of idt
            offset: self, //pointer to idt
        };

        unsafe {
            asm!("lidt [{0:e}]", in(reg) &descriptor);
        }
    }
}

impl IdtEntry {
    pub fn new(offset: u32) -> Self {

        let offset_low: u16 = ((offset << 16) >> 16) as u16; //calculate lower 16 bits of offset

        let offset_high: u16 = (offset >> 16) as u16; //calculate higher 16 bits of offset

        let segment_selector: u16 = {   //segment selector of gdt entry
            let rpl = 0b00 << 0;        //ring privilege level (0 for ring 0)
            let ti = 0b0 << 2;          //0 to use gdt, 1 to use ldt
            let index = 0b1 << 3;       //entry index of gdt code segment, in my case code entry is the second (0b1)

            rpl | ti | index
        };

        let reserved: u8 = 0; //always zero

        let flags: u8 = { //entry flags
            let gate_type = 0xe << 0;   //gate type, 0xe for 32bit interrupt gate, 0xf for 32bit trap gate
            let zero = 0 << 3;          //always zero  
            let dpl = 0 << 5;           //ring allowed to use this interrupt
            let p = 1 << 7;             //presence bit, 1 to enable

            gate_type | zero | dpl | p
        };

        Self {
            offset_low: offset_low,
            segment_selector: segment_selector,
            reserved: reserved,
            flags: flags,
            offset_high: offset_high,
        }
    }
}