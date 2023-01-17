#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

static TEXT: &'static str = "world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello, {}!", TEXT);
  panic!("Goodbye, {}!", TEXT);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}
