use crate::syscall::sys_exit;
use core::alloc::Layout;
use core::panic::PanicInfo;

#[linkage = "weak"]
#[no_mangle]
fn main() -> usize {
    panic!("No main() linked");
}

use crate::DYNAMIC_ALLOCATOR;

fn initialize() {
    const HEAP_SIZE: usize = 0x1000;
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    unsafe {
        DYNAMIC_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, HEAP_SIZE);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();
    let message = info.message().unwrap();
    println!(
        "Panic in {} at line {}: {}",
        location.file(),
        location.line(),
        message
    );
    loop {}
}

#[no_mangle]
pub extern "C" fn _start(_args: isize, _argv: *const u8) -> ! {
    initialize();
    sys_exit(main())
}

#[no_mangle]
pub extern "C" fn abort() {
    panic!("Abort");
}

#[lang = "oom"]
fn oom(_: Layout) -> ! {
    panic!("Out of memory!");
}
