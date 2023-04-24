#[repr(C, packed)]
pub struct TaskStateSegment {
    link: u32,
    esp0: u32,
    ss0: u32, //16 bit
    esp1: u32,
    ss1: u32, //16 bit
    esp2: u32,
    ss2: u32, //16 bit
    cr3: u32,
    eip: u32,
    eflags: u32,
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    es: u32,   //16 bit
    cs: u32,   //16 bit
    ss: u32,   //16 bit
    ds: u32,   //16 bit
    fs: u32,   //16 bit
    gs: u32,   //16 bit
    ldtr: u32, //16 bit
    iopb: u32, //higher 16 bit
    ssp: u32,
}

impl TaskStateSegment {
    pub fn new(address: u32) -> Self {
        Self {
            link: 0,
            esp0: 0,
            ss0: 0, //16 bit
            esp1: 0,
            ss1: 0, //16 bit
            esp2: 0,
            ss2: 0, //16 bit
            cr3: 0,
            eip: address,
            eflags: 0,
            eax: 0,
            ecx: 0,
            edx: 0,
            ebx: 0,
            esp: 0,
            ebp: 0,
            esi: 0,
            edi: 0,
            es: 0x0010, //16 bit
            cs: 0x0008, //16 bit
            ss: 0x0010, //16 bit
            ds: 0x0010, //16 bit
            fs: 0,      //16 bit
            gs: 0,      //16 bit
            ldtr: 0,    //16 bit
            iopb: 0,    //higher 16 bit
            ssp: 0,
        }
    }
}
