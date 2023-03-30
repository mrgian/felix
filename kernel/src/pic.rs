use core::arch::asm;

const PRIMARY_PIC_COMMAND_PORT: u8 = 0x20;
const PRIMARY_PIC_DATA_PORT: u8 = 0x21;

const SECONDARY_PIC_COMMAND_PORT: u8 = 0xa0;
const SECONDARY_PIC_DATA_PORT: u8 = 0xa1;

const COMMAND_INIT: u8 = 0x11;
const COMMAND_EOF: u8 = 0x20;

const MODE: u8 = 0x01;

const INT_OFFSET: u8 = 32;

struct Pic {
    offset: u8,
    command_port: u8,
    data_port: u8,
}

impl Pic {
    pub fn read_data(&self) -> u8 {
        let data: u8;
        unsafe {
            asm!("in al, dx", out("al") data, in("dx") self.data_port as u16);
        }

        data
    }

    pub fn write_data(&self, data: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.data_port as u16, in("al") data);
        }
    }

    pub fn send_command(&self, command: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.command_port as u16, in("al") command);
        }
    }

    pub fn end_interrupt(&self) {
        unsafe {
            asm!("out dx, al", in("dx") self.command_port as u16, in("al") COMMAND_EOF);
        }
    }

    pub fn handles_interrupt(&self, interupt: u8) -> bool {
        self.offset <= interupt && interupt < self.offset + 8
    }
}

pub struct Pics {
    primary: Pic,
    secondary: Pic,
}

impl Pics {
    pub fn new() -> Self {
        let primary_pic = Pic {
            offset: INT_OFFSET,
            command_port: PRIMARY_PIC_COMMAND_PORT,
            data_port: PRIMARY_PIC_DATA_PORT,
        };

        let secondary_pic = Pic {
            offset: INT_OFFSET + 8, //each pic can handle 8 interrupts, so secondary pic handles the ints after the primary ones
            command_port: SECONDARY_PIC_COMMAND_PORT,
            data_port: SECONDARY_PIC_DATA_PORT,
        };

        Self {
            primary: primary_pic,
            secondary: secondary_pic,
        }
    }

    pub fn init(&self) {
        let mask1 = self.primary.read_data();
        let mask2 = self.secondary.read_data();

        self.primary.send_command(COMMAND_INIT);
        wait();
        self.secondary.send_command(COMMAND_INIT);
        wait();

        self.primary.write_data(self.primary.offset);
        wait();
        self.secondary.write_data(self.secondary.offset);
        wait();

        self.primary.write_data(4);
        wait();
        self.secondary.write_data(2);
        wait();

        self.primary.write_data(MODE);
        wait();
        self.secondary.write_data(MODE);
        wait();

        self.primary.write_data(mask1);
        self.secondary.write_data(mask2);
    }

    pub fn handles_interrupt(&self, interrupt: u8) -> bool {
        self.primary.handles_interrupt(interrupt) || self.secondary.handles_interrupt(interrupt)
    }

    pub fn end_interrupt(&self, interrupt: u8) {
        if self.handles_interrupt(interrupt) {
            if self.secondary.handles_interrupt(interrupt) {
                self.secondary.end_interrupt();
            }
            self.primary.end_interrupt();
        }
    }
}

pub fn wait() {
    unsafe {
        asm!("out dx, al", in("dx") 0x80 as u16, in("al") 0 as u8);
    }
}