#![no_std]
#![no_main]

use core::panic::PanicInfo;

use simple_os::{println, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}, this is vga display!!!", "!");
    serial_println!("this is from serial{}", "...");

    simple_os::init();

    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // }

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    println!("not crash?!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
