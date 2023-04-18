pub static mut PAGE_DIRECTORY: PageDirectory = PageDirectory { entries: [0x00000002; 1024] };

#[repr(align(4096))]
pub struct PageDirectory {
    pub entries: [u32; 1024],
}

impl PageDirectory {
    pub fn init(&mut self) {
        let table = PageTable::new();

        self.entries[0] = (&table as *const PageTable) as u32;
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
            table.entries[i] = ((i * 0x1000) | 3) as u32;
        }

        table
    }
}