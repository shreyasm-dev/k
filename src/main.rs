#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(k::test_runner)]
#![reexport_test_harness_main = "test_main"]

use k::println;

static TEXT: &'static str = "world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  #[cfg(test)]
  test_main();

  println!("Hello, {}!", TEXT);
  println!("Goodbye, {}!", TEXT);

  loop {}
}
