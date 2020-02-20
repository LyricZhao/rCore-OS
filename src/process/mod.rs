use crate::process::thread::Thread;

mod stack;
mod thread;

#[no_mangle]
pub extern "C" fn temp_thread(from: &mut Thread, current: &mut Thread) {
    println!("Hello world! (from temp_thread)");
    current.switch_to(from);
}

pub fn initialize() {
    let mut boot_thread = Thread::get_boot_thread();
    let mut temp_thread = Thread::new_kernel(temp_thread as usize);

    temp_thread.append_initial_arguments([
        &*boot_thread as *const Thread as usize,
        &*temp_thread as *const Thread as usize,
        0,
    ]);
    boot_thread.switch_to(&mut temp_thread);
    println!("Switch back to boot thread");
    loop {}
}
