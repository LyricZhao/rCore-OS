use crate::process::pool::ThreadPool;
use crate::process::processor::Processor;
use crate::process::scheduler::RoundRobinScheduler;
use crate::process::thread::Thread;
use alloc::boxed::Box;

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

pub fn initialize() {
    let scheduler = RoundRobinScheduler::new(2);
    let pool = ThreadPool::new(128, Box::new(scheduler));
    let idle = Thread::new_kernel(Processor::idle_main as usize);
    idle.append_args([&PROCESSOR as *const Processor as usize, 0, 0]);
    PROCESSOR.initialize(idle, Box::new(pool));

    for i in 0..5 {
        println!("Adding {}", i);
        PROCESSOR.add_thread({
            let thread = Thread::new_kernel(test_thread as usize);
            thread.append_args([i, 0, 0]);
            thread
        });
    }

    extern "C" {
        fn _user_img_start();
        fn _user_img_end();
    }

    let data = unsafe {
        core::slice::from_raw_parts(
            _user_img_start as *const u8,
            _user_img_end as usize - _user_img_start as usize,
        )
    };
    let user_thread = Thread::new_user(data);
    PROCESSOR.add_thread(user_thread);
}
