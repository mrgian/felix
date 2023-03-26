use core::mem::size_of;
use core::arch::asm;

//const IDT_ENTRIES: usize = 256;
const IDT_ENTRIES: usize = 16;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset_low: u16,
    segment_selector: u16,
    reserved: u8,
    flags: u8,
    offset_high: u16,
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
            offset: self,
            size: (IDT_ENTRIES * size_of::<IdtEntry>() - 1) as u16, //calculate size of idt
        };

        for i in 0..16 {
            self.entries[i].print();
        }

        unsafe {
            asm!("lidt [{0:e}]", in(reg) &descriptor);
        }
    }
}

impl IdtEntry {
    pub fn new(offset: u32) -> Self {

        let offset_low: u16 = ((offset << 16) >> 16) as u16;

        let offset_high: u16 = (offset >> 16) as u16;

        let segment_selector: u16 = {
            let rpl = 0b00 << 0;
            let ti = 0b0 << 2;
            let index = 0x0000_0000 << 6;

            rpl | ti | index
        };

        let reserved: u8 = 0;

        let flags: u8 = {
            let gate_type = 0xf << 0;
            let zero = 0 << 3;
            let dpl = 0 << 5;
            let p = 1 << 7;

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

    pub fn print(&self) {
        let low = self.offset_low;
        let sel = self.segment_selector;
        let res = self.reserved;
        let flags = self.flags;
        let high = self.offset_high;

        println!("Low: {:X}, Select: {:X}, Res: {:X}, Flags: {:X}, High: {:X}", low, sel, res, flags, high);
    }
}