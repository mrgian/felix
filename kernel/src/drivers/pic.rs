use core::arch::asm;

//PICs DRIVER
//Used to initialize and configure pics

//define a global PICS so it can be accessed from everywhere
pub static PICS: Pics = Pics {
    master: Pic {
        offset: OFFSET,
        command_port: MASTER_PIC_COMMAND_PORT,
        data_port: MASTER_PIC_DATA_PORT,
    },
    slave: Pic {
        offset: OFFSET + IRQ_COUNT, //each pic can handle 8 interrupts, so slave pic handles the ints after the master ones
        command_port: SLAVE_PIC_COMMAND_PORT,
        data_port: SLAVE_PIC_DATA_PORT,
    },
};

//master pic ports
const MASTER_PIC_COMMAND_PORT: u8 = 0x20;
const MASTER_PIC_DATA_PORT: u8 = 0x21;

//slave pic ports
const SLAVE_PIC_COMMAND_PORT: u8 = 0xa0;
const SLAVE_PIC_DATA_PORT: u8 = 0xa1;

//command bytes
const COMMAND_INIT: u8 = 0x11;
const COMMAND_EOF: u8 = 0x20;

//pic 8086 mode
const MODE: u8 = 0x01;

//where to start remapping IRQs, start from 32 because 0-31 interrupts are used by CPU exceptions
const OFFSET: u8 = 32;

//how many interrupts a pic can handle
const IRQ_COUNT: u8 = 8;

//single programmable interrupt controller
struct Pic {
    offset: u8,
    command_port: u8,
    data_port: u8,
}

impl Pic {
    //read byte from pic data port
    pub fn read_data(&self) -> u8 {
        let data: u8;
        unsafe {
            asm!("in al, dx", out("al") data, in("dx") self.data_port as u16);
        }

        data
    }

    //write byte to pic data port
    pub fn write_data(&self, data: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.data_port as u16, in("al") data);
        }
    }

    //send command to pic
    pub fn send_command(&self, command: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.command_port as u16, in("al") command);
        }
    }

    //notify pic that interrupts has ended
    pub fn end_interrupt(&self) {
        unsafe {
            asm!("out dx, al", in("dx") self.command_port as u16, in("al") COMMAND_EOF);
        }
    }

    //check if pic is handling interrupt
    pub fn handles_interrupt(&self, interupt: u8) -> bool {
        self.offset <= interupt && interupt < self.offset + IRQ_COUNT
    }
}

//two pics chained one after another
pub struct Pics {
    master: Pic,
    slave: Pic,
}

impl Pics {
    //initialize both pics
    pub fn init(&self) {
        //backup masks, need to restore later
        let mask1 = self.master.read_data();
        let mask2 = self.slave.read_data();

        //send init command
        self.master.send_command(COMMAND_INIT);
        wait();
        self.slave.send_command(COMMAND_INIT);
        wait();

        //set offset
        self.master.write_data(self.master.offset);
        wait();
        self.slave.write_data(self.slave.offset);
        wait();

        //tell master pic that there is a connected slave pic on IRQ2
        self.master.write_data(4);
        wait();
        //tell slave pic that he is slave
        self.slave.write_data(2);
        wait();

        //set 8086 mode
        self.master.write_data(MODE);
        wait();
        self.slave.write_data(MODE);
        wait();

        //restore mask
        self.master.write_data(mask1);
        self.slave.write_data(mask2);
    }

    //check if one of the pics is handling an interrupt
    pub fn handles_interrupt(&self, interrupt: u8) -> bool {
        self.master.handles_interrupt(interrupt) || self.slave.handles_interrupt(interrupt)
    }

    //notify pics that current interrupt has ended
    pub fn end_interrupt(&self, interrupt: u8) {
        if self.handles_interrupt(interrupt) {
            if self.slave.handles_interrupt(interrupt) {
                self.slave.end_interrupt();
            }
            self.master.end_interrupt();
        }
    }
}

//since pics may not be very fast, we need a wait function
//this simply writes data to an unused port, should take some time
pub fn wait() {
    unsafe {
        asm!("out dx, al", in("dx") 0x80 as u16, in("al") 0 as u8);
    }
}
