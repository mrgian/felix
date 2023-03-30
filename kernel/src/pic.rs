const PRIMARY_PIC_COMMAND_PORT: u8 = 0x20;
const PRIMARY_PIC_DATA_PORT: u8 = 0x21;

const SECONDARY_PIC_COMMAND_PORT: u8 = 0xa0;
const SECONDARY_PIC_DATA_PORT: u8 = 0xa1;

const COMMAND_INIT: u8 = 0x11;
const COMMAND_EOF: u8 = 0x20;

const MODE: u8 = 0x01;

struct Pic {
    offset: u8,
    command_port: u8,
    data_port: u8,
}

impl Pic {
    pub fn read_data(&self) -> u8 {

    }

    pub fn write_data(&self, data: u8) {

    }

    pub fn send_command(&self, command: u8) {
        
    }
}

struct Pics {
    pics: [Pic; 2],
}

impl Pics {
    pub fn new(offset: u8) -> Self {
        let primary_pic = Pic {
            offset: offset,
            command_port: PRIMARY_PIC_COMMAND_PORT,
            data_port: PRIMARY_PIC_DATA_PORT,
        }

        let secondary_pic = Pic {
            offset: offset + 8, //each pic can handle 8 interrupts, so secondary pic handles the ints after the primary ones
            command_port: PRIMARY_PIC_COMMAND_PORT,
            data_port: PRIMARY_PIC_DATA_PORT,
        }

        Self {
            pics: [primary_pic, secondary_pic],
        }
    }
}