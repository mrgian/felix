use core::arch::asm;

pub static mut PAGE_DIRECTORY: PageDirectory = PageDirectory {
    //0b010 (supervisor, write, not present)
    entries: [0x00000002; 1024],
};

#[repr(align(4096))]
pub struct PageDirectory {
    pub entries: [u32; 1024],
}

impl PageDirectory {
    pub fn init(&mut self) {
        let table = PageTable::new();

        self.entries[0] = (&table as *const PageTable) as u32 | 0b011;
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
    pub fn new() -> Self {
        let mut table = Self { entries: [0; 1024] };

        for i in 0..1024 {
            //0b011 (supervisor, write, present)
            table.entries[i] = ((i * 0x1000) | 0b011) as u32;
        }

        table
    }
}
