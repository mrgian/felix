//CPU SCHEDULER

//each task has a 4KiB stack containg the cpu state in the bottom part of it
pub struct Task {
    stack: [u8; 4096],
    cpu_state: *mut CPUState,
}

impl Task {
    pub fn new(entry: u32) -> Self {
        //init null task
        let mut task = Task {
            stack: [0; 4096],
            cpu_state: 0 as *mut CPUState,
        };

        //set cpu state pointer to the bottom part of its stack
        let mut state = &task.stack as *const u8;
        unsafe {
            state = state.byte_add(4096);
            state = state.byte_sub(core::mem::size_of::<CPUState>());
        }

        //update cpu state pointer
        task.cpu_state = state as *mut CPUState;

        unsafe {
            //init registers
            (*(task.cpu_state)).eax = 0;
            (*(task.cpu_state)).ebx = 0;
            (*(task.cpu_state)).ecx = 0;
            (*(task.cpu_state)).edx = 0;
            (*(task.cpu_state)).esi = 0;
            (*(task.cpu_state)).edi = 0;
            (*(task.cpu_state)).ebp = 0;

            //set instruction pointer to entry point of task
            (*(task.cpu_state)).eip = entry;

            //set code segment
            (*(task.cpu_state)).cs = 0x8;

            //set eflags
            (*(task.cpu_state)).eflags = 0x202;
        }

        //return new task
        task
    }
}

pub struct TaskManager {
    tasks: [*mut Task; 256], //arry of pointers to tasks
    task_count: i8, //how many tasks are in the queue
    current_task: i8, //current running task
}

//init null task manager
pub static mut TASK_MANAGER: TaskManager = TaskManager {
    tasks: [0 as *mut Task; 256],
    task_count: 0,
    current_task: -1,
};

impl TaskManager {
    //add given task to next slot
    pub fn add_task(&mut self, task: *mut Task) {
        self.tasks[self.task_count as usize] = task;
        self.task_count += 1;
    }

    //triggers scheduler with round robin scheduling algorithm, returns new cpu state
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
    //manually pushed
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
    ebp: u32,

    //automatically pushed by cpu
    eip: u32,
    cs: u32,
    eflags: u32,
    esp: u32,
    ss: u32,
}
