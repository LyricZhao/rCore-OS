use crate::process::pool::ThreadPool;
use crate::process::processor::Processor;
use crate::process::scheduler::RoundRobinScheduler;
use crate::process::thread::Thread;
use alloc::boxed::Box;
use crate::fs::{ROOT_INODE, INodeExt};

mod context;
mod elf;
mod pool;
mod processor;
mod scheduler;
mod stack;
mod thread;

pub type ThreadID = usize;
pub type ExitCode = usize;

static PROCESSOR: Processor = Processor::new();

#[no_mangle]
pub extern "C" fn test_thread(arg: usize) -> ! {
    println!("Begin of thread {}", arg);
    for _i in 0..800 {
        print!("{}", arg);
    }
    println!("\nEnd  of thread {}", arg);
    exit(0);
}

pub fn run() {
    PROCESSOR.run();
}

pub fn exit(code: ExitCode) -> ! {
    PROCESSOR.exit(code)
}

pub fn tick() {
    PROCESSOR.tick();
}

pub fn sleep() {
    PROCESSOR.sleep();
}

pub fn wake_up(id: usize) {
    PROCESSOR.wake_up(id);
}

pub fn current_tid() -> usize {
    PROCESSOR.current_tid()
}

pub fn initialize() {
    let scheduler = RoundRobinScheduler::new(2);
    let pool = ThreadPool::new(128, Box::new(scheduler));
    let idle = Thread::new_kernel(Processor::idle_main as usize);
    idle.append_args([&PROCESSOR as *const Processor as usize, 0, 0]);
    PROCESSOR.initialize(idle, Box::new(pool));

    // User shell
    execute("rust/shell", None);

    /*
    // Kernel thread test
    for i in 0..5 {
        PROCESSOR.add_thread({
            let thread = Thread::new_kernel(test_thread as usize);
            thread.append_args([i, 0, 0]);
            thread
        });
    }
    */
}

pub fn execute(path: &str, host: Option<ThreadID>) -> bool {
    let found = ROOT_INODE.lookup(path);
    match found {
        Ok(inode) => {
            let data = inode.read_as_vec().unwrap();
            let thread = Thread::new_user(data.as_slice(), host);
            PROCESSOR.add_thread(thread);
            true
        },
        Err(_) => {
            println!("Program not found.");
            false
        }
    }
}
