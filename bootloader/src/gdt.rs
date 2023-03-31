use core::arch::asm;
use core::mem::size_of;

//GLOBAL DESCRIPTOR TABLE
//Warning! Mutable static here
//TODO: Implement a mutex to get safe access to this
pub static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable {
    entries: [NULL_ENTRY; GDT_ENTRIES],
};

const GDT_ENTRIES: usize = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct GdtEntry {
    entry: u64,
}

static NULL_ENTRY: GdtEntry = GdtEntry { entry: 0 };

#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    entries: [GdtEntry; GDT_ENTRIES],
}

#[repr(C, packed)]
pub struct GdtDescriptor {
    size: u16,                            //gdt size
    offset: *const GlobalDescriptorTable, //pointer to gdt
}

//global descriptor table for flat memory model
impl GlobalDescriptorTable {
    //left shifts are used to set bit from specified position
    pub fn init(&mut self) {
        //segment lenght (0xffff means all 32bit memory)
        let limit = {
            let limit_low = 0xffff << 0;
            let limit_high = 0xf << 48;

            limit_low | limit_high
        };

        //base address
        let base = {
            let base_low = 0x0000 << 16;
            let base_high = 0x00 << 56;

            base_low | base_high
        };

        //access byte
        let access = {
            let p = 0b1 << 47; //present bit (1 for any segment)
            let dpl = 0b00 << 46; //descriptor privilege level (ring, 0 for highest privilege, 3 for lowest)
            let s = 0b1 << 44; //descriptor type bit
            let e = 0b0 << 43; //executable bit
            let dc = 0b0 << 42; //direction bit/conforming bit
            let rw = 0b1 << 41; //readable bit/writable bit
            let a = 0b0 << 40; //accessed bit

            p | dpl | s | e | dc | rw | a
        };

        //flags
        let flags = {
            let g = 0b1 << 55; //granularity flag
            let db = 0b1 << 54; //size flag
            let l = 0b0 << 53; //long mode flag
            let r = 0b0 << 52; //reserved

            g | db | l | r
        };

        let executable = 0b1 << 43; //set only executable flag again, instead of setting all values again

        //first entry is always zero
        //second entry is code segment (default + executable)
        //third entry is data segment (default)
        let zero = GdtEntry { entry: 0 };
        let code = GdtEntry {
            entry: limit | base | access | flags | executable,
        };
        let data = GdtEntry {
            entry: limit | base | access | flags,
        };

        self.entries = [zero, code, data];
    }

    //load gdt using lgdt instruction
    pub fn load(&self) {
        let descriptor = GdtDescriptor {
            size: (GDT_ENTRIES * size_of::<GdtEntry>() - 1) as u16, //calculate size of gdt
            offset: self,                                           //pointer to gdt
        };

        unsafe {
            asm!("lgdt [{0:e}]", in(reg) &descriptor);
        }
    }
}
