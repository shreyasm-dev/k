#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use core::fmt::Write;
use vga::{WRITER};

static TEXT: &'static str = "world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  write!(WRITER.lock(), "Hello, {}!\nGoodbye, {}!", TEXT, TEXT).unwrap();

  loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}
