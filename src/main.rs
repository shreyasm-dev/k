#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(k::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use k::{halt, init, println};

#[cfg(test)]
use k::test_panic_handler;

static TEXT: &'static str = "world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello, {}!", TEXT);
  println!("Goodbye, {}?", TEXT);

  init();

  #[cfg(test)]
  test_main();

  halt();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  halt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  test_panic_handler(info)
}
