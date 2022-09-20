#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}, this is vga display!!!", "!");
    serial_println!("this is from serial{}", "...");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
