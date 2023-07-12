//TASK MANAGER

const STACK_SIZE: usize = 4096;
const MAX_TASKS: i8 = 32;

//each task has a 4KiB stack containg the cpu state in the bottom part of it
#[derive(Copy, Debug, Clone)]
pub struct Task {
    pub stack: [u8; STACK_SIZE],
    pub cpu_state_ptr: u32, //pub cpu_state: *mut CPUState,
    pub running: bool,
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

static mut TASKS: [Task; MAX_TASKS as usize] = [NULL_TASK; MAX_TASKS as usize];

static NULL_TASK: Task = Task {
    stack: [0; STACK_SIZE],
    cpu_state_ptr: 0 as u32, //cpu_state: 0 as *mut CPUState,
    running: false,
};

impl Task {
    //setup task stack, zeroing its cpu state and setting entry point
    pub fn init(&mut self, entry_point: u32) {
        //mark task as running
        self.running = true;

        //set cpu state pointer to the bottom part of its stack
        let mut state = &self.stack as *const u8;
        unsafe {
            state = state.byte_add(STACK_SIZE);
            state = state.byte_sub(core::mem::size_of::<CPUState>());
        }

        //update cpu state pointer
        self.cpu_state_ptr = state as u32;

        let cpu_state = self.cpu_state_ptr as *mut CPUState;

        unsafe {
            //init registers
            (*cpu_state).eax = 0;
            (*cpu_state).ebx = 0;
            (*cpu_state).ecx = 0;
            (*cpu_state).edx = 0;
            (*cpu_state).esi = 0;
            (*cpu_state).edi = 0;
            (*cpu_state).ebp = 0;

            //set instruction pointer to entry point of task
            (*cpu_state).eip = entry_point;

            //set code segment
            (*cpu_state).cs = 0x8;

            //set eflags
            (*cpu_state).eflags = 0x202;
        }
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
    pub fn init(&mut self) {
        unsafe {
            TASKS[0].init(idle as u32);
            self.add_task(&mut TASKS[0] as *mut Task);
        }
    }

    //add given task to next slot
    pub fn add_task(&mut self, task: *mut Task) {
        let free_slot = self.get_free_slot();

        self.tasks[free_slot as usize] = task;
        self.task_count += 1;
    }

    //remove task
    pub fn remove_task(&mut self, id: usize) {
        if id != 0 {
            self.tasks[id] = 0 as *mut Task;
            self.task_count -= 1;
        }
    }

    pub fn remove_current_task(&mut self) {
        self.remove_task(self.current_task as usize);
    }

    //CPU SCHEDULER LOGIC
    //triggers scheduler with round robin scheduling algorithm, returns new cpu state
    pub fn schedule(&mut self, cpu_state: *mut CPUState) -> *mut CPUState {
        unsafe {
            //if no tasks return current state
            if self.task_count <= 0 {
                return cpu_state;
            }

            //save current state of current task
            if self.current_task >= 0 {
                (*(self.tasks[self.current_task as usize])).cpu_state_ptr = cpu_state as u32;
            }

            self.current_task = self.get_next_task();

            (*(self.tasks[self.current_task as usize])).cpu_state_ptr as *mut CPUState
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
            for i in 0..MAX_TASKS {
                let running = (*(self.tasks[i as usize])).running;
                if running == false {
                    slot = i as i8;
                    return slot;
                }
            }
        }

        slot
    }

    pub fn list_tasks(&self) {
        libfelix::println!("Running tasks:");

        unsafe {
            for i in 0..MAX_TASKS {
                let running = (*(self.tasks[i as usize])).running;
                if running {
                    libfelix::println!("ID: {}", i);
                }
            }
        }
    }

    pub fn run_app(&mut self, app_entry_point: u32) {
        unsafe {
            TASKS[4].init(app_entry_point as u32);
            self.add_task(&mut TASKS[4] as *mut Task);
        }
    }

    pub fn add_dummy_task_a(&mut self) {
        unsafe {
            TASKS[1].init(task_a as u32);
            self.add_task(&mut TASKS[1] as *mut Task);
        }
    }

    pub fn add_dummy_task_b(&mut self) {
        unsafe {
            TASKS[2].init(task_b as u32);
            self.add_task(&mut TASKS[2] as *mut Task);
        }
    }

    pub fn add_dummy_task_c(&mut self) {
        unsafe {
            TASKS[3].init(task_c as u32);
            self.add_task(&mut TASKS[3] as *mut Task);
        }
    }
}

fn idle() {
    loop{}
}

//EXAMPLE TASKS
fn task_a() {
    let mut a: u32 = 0;
    let mut b: u8 = 0;
    loop {
        if a == 100_000_000 {
            libfelix::println!("Process A running. {}% complete.", b);
            a = 0;
            b += 1;

            if b == 100 {
                libfelix::println!("Process A complete.");
                break;
            }
        }
        a += 1;
    }
    loop{}
}

fn task_b() {
    let mut a: u32 = 0;
    let mut b: u8 = 0;
    loop {
        if a == 100_000_000 {
            libfelix::println!("Process B running. {}% complete.", b);
            a = 0;
            b += 1;

            if b == 100 {
                libfelix::println!("Process B complete.");
                break;
            }
        }
        a += 1;
    }
    loop{}
}

fn task_c() {
    let mut a: u32 = 0;
    let mut b: u8 = 0;
    loop {
        if a == 100_000_000 {
            libfelix::println!("Process C running. {}% complete.", b);
            a = 0;
            b += 1;

            if b == 100 {
                libfelix::println!("Process C complete.");
                break;
            }
        }
        a += 1;
    }
    loop{}
}
