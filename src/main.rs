#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod test;

use core::panic::PanicInfo;

#[cfg(test)]
use crate::test::assert;

static TEXT: &'static str = "world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  #[cfg(test)]
  assert(1, 2);

  println!("Hello, {}!", TEXT);
  panic!("Goodbye, {}!", TEXT);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
  println!("Running {} tests", tests.len());
  for test in tests {
    test();
  }
}
