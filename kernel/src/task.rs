pub struct Task {
    stack: [u8; 4096],
    cpu_state: *mut CPUState,
}

impl Task {
    pub fn new(entry: u32) -> Self {
        let mut task = Task {
            stack: [0; 4096],
            cpu_state: 0 as *mut CPUState,
        };

        let mut state = &task.stack as *const u8;
        unsafe {
            state = state.byte_add(4096);
            state = state.byte_sub(core::mem::size_of::<CPUState>());
        }

        task.cpu_state = state as *mut CPUState;

        unsafe {
            (*(task.cpu_state)).eax = 0;
            (*(task.cpu_state)).ebx = 0;
            (*(task.cpu_state)).ecx = 0;
            (*(task.cpu_state)).edx = 0;

            (*(task.cpu_state)).esi = 0;
            (*(task.cpu_state)).edi = 0;
            (*(task.cpu_state)).ebp = 0;

            /*(*(task.cpu_state)).gs = 0;
            (*(task.cpu_state)).fs = 0;
            (*(task.cpu_state)).es = 0;
            (*(task.cpu_state)).ds = 0;*/

            (*(task.cpu_state)).eip = entry;
            (*(task.cpu_state)).cs = 0x8;
            (*(task.cpu_state)).eflags = 0x202;
        }

        task
    }
}

pub struct TaskManager {
    tasks: [*mut Task; 256],
    task_count: i8,
    current_task: i8,
}

pub static mut TASK_MANAGER: TaskManager = TaskManager {
    tasks: [0 as *mut Task; 256],
    task_count: 0,
    current_task: -1,
};

impl TaskManager {
    pub fn add_task(&mut self, task: *mut Task) {
        self.tasks[self.task_count as usize] = task;
        self.task_count += 1;
    }

    pub fn schedule(&mut self, cpu_state: *mut CPUState) -> *mut CPUState {
        unsafe {
            if self.task_count <= 0 {
                return cpu_state;
            }

            if self.current_task >= 0 {
                (*(self.tasks[self.current_task as usize])).cpu_state = cpu_state;
            }

            self.current_task += 1;

            if self.current_task >= self.task_count {
                self.current_task %= self.task_count;
            }

            (*(self.tasks[self.current_task as usize])).cpu_state
        }
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

    /*gs: u32,
    fs: u32,
    es: u32,
    ds: u32,*/
    error: u32,

    eip: u32,
    cs: u32,
    eflags: u32,
    esp: u32,
    ss: u32,
}
