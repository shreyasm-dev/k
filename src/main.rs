#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod qemu;
mod serial;
mod test;
mod tests;

use core::panic::PanicInfo;
use crate::qemu::{exit_qemu, QemuExitCode};

static TEXT: &'static str = "world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  #[cfg(test)]
  test_main();

  println!("Hello, {}!", TEXT);
  println!("Goodbye, {}!", TEXT);

  loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  serial_println!("[failed]");
  serial_println!("Error: {}", info);
  exit_qemu(QemuExitCode::Failed);
  loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn() -> bool]) {
  let mut passed = true;

  println!("Running {} tests", tests.len());
  for test in tests {
    passed = passed && test();
  }

  exit_qemu(if passed { QemuExitCode::Success } else { QemuExitCode::Failed });
}
