pub struct Task {
    stack: [u8; 4096],
    cpu_state: &CPUState 
}

impl Task {
    pub fn new(entry: u32) -> Self {
        Self {

        }
    }
}

pub struct TaskManager {
    tasks: [Task; 256],
    task_count: u8,
    current_task: u8,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn add_task(&self, task: &Task) {

    }

    pub fn schedule(&self, cpu_state: &CPUState) {

    }
}

#[repr(C, packed)]
pub struct CPUState {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,

    esi: u32,
    edi: u32,
    ebp: u32,

    gs: u32,
    fs: u32,
    es: u32,
    ds: u32,

    error: u32,

    eip: u32,
    cs: u32,
    eflags: u32,
    esp: u32,
    ss: u32,
}