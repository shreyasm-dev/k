#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

static TEXT: &'static str = "world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello, {}!", TEXT);
  println!("Goodbye, {}!", TEXT);

  loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}
