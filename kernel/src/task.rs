//CPU SCHEDULER

const MAX_TASKS: i8 = 127;

//each task has a 4KiB stack containg the cpu state in the bottom part of it
pub struct Task {
    stack: [u8; 4096],
    cpu_state: *mut CPUState,
    running: bool,
}

impl Task {
    pub fn new(entry: u32) -> Self {
        //init null task
        let mut task = Task {
            stack: [0; 4096],
            cpu_state: 0 as *mut CPUState,
            running: true,
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
    tasks: [*mut Task; MAX_TASKS as usize], //arry of pointers to tasks
    task_count: i8,                         //how many tasks are in the queue
    current_task: i8,                       //current running task
}

//init null task manager
pub static mut TASK_MANAGER: TaskManager = TaskManager {
    tasks: [0 as *mut Task; MAX_TASKS as usize],
    task_count: 0,
    current_task: -1,
};

impl TaskManager {
    //add given task to next slot
    pub fn add_task(&mut self, task: *mut Task) {
        let free_slot = self.get_free_slot();

        self.tasks[free_slot as usize] = task;
        self.task_count += 1;
    }

    //remove task
    pub fn remove_task(&mut self, id: usize) {
        self.tasks[id] = 0 as *mut Task;
        self.task_count -= 1;
    }

    pub fn remove_current_task(&mut self) {
        self.remove_task(self.current_task as usize);
    }

    //triggers scheduler with round robin scheduling algorithm, returns new cpu state
    pub fn schedule(&mut self, cpu_state: *mut CPUState) -> *mut CPUState {
        unsafe {
            //if no tasks return current state
            if self.task_count <= 0 {
                return cpu_state;
            }

            //save current state of current task
            if self.current_task >= 0 {
                (*(self.tasks[self.current_task as usize])).cpu_state = cpu_state;
            }

            self.current_task = self.get_next_task();

            (*(self.tasks[self.current_task as usize])).cpu_state
        }
    }

    pub fn get_next_task(&self) -> i8 {
        unsafe {
            let mut i = self.current_task + 1;
            while i < MAX_TASKS {
                let running = (*(self.tasks[i as usize])).running;

                if running {
                    return i;
                }

                i = (i + 1) % MAX_TASKS;
            }
        }

        -1
    }

    pub fn get_free_slot(&self) -> i8 {
        let mut slot: i8 = -1;

        unsafe {
            for i in 0..127 {
                let running = (*(self.tasks[i])).running;
                if running == false {
                    slot = i as i8;
                    return slot;
                }
            }
        }

        slot
    }

    pub fn list_tasks(&self) {
        stdio::println!("Running tasks:");

        unsafe {
            for i in 0..127 {
                let running = (*(self.tasks[i])).running;
                if running {
                    stdio::println!("ID: {}", i);
                }
            }
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
