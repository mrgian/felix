use core::arch::asm;

pub static mut PAGING: PageDirectory = PageDirectory {
    //0b010 (supervisor, write, not present)
    entries: [0x00000002; 1024],
};

#[repr(align(4096))]
pub struct PageDirectory {
    pub entries: [u32; 1024],
}

impl PageDirectory {
    pub fn set_table(&mut self, index: usize, table: &PageTable) {
        self.entries[index] = (table as *const PageTable) as u32 | 0b011;
    }

    pub fn enable(&self) {
        unsafe {
            let address = (self as *const PageDirectory) as u32;

            asm!("mov cr3, eax",
                "mov eax, cr0",
                "or eax, 0x80000001",
                "mov cr0, eax",
                in("eax") address);
        }
    }
}

#[repr(align(4096))]
pub struct PageTable {
    pub entries: [u32; 1024],
}

impl PageTable {
    pub fn new(from: u32) -> Self {
        let mut table = Self { entries: [0; 1024] };

        for i in 0..1024 {
            //0b011 (supervisor, write, present)
            table.entries[i] = (((i * 0x1000) + from as usize) | 0b011) as u32;
        }

        table
    }

    pub fn test() -> Self {
        let mut table = Self { entries: [0; 1024] };
        for i in 0..4 {
            //0b011 (supervisor, write, present)
            table.entries[i] = (((i * 0x1000) + 0x0050_0000) | 0b011) as u32;
        }

        table
    }
}
