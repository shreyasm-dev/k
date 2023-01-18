#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga;
pub mod qemu;
pub mod serial;
pub mod test;

use core::panic::PanicInfo;
use crate::qemu::{exit_qemu, QemuExitCode};

pub fn test_runner(tests: &[&dyn Fn() -> (&'static str, bool)]) {
  let results = tests.iter().map(|test| test());

  for (name, passed) in results {
    if passed {
      serial_println!("[ok] {}", name);
    } else {
      serial_println!("[failed] {}", name);
    }
  }

  exit_qemu(QemuExitCode::Success);
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
  serial_println!("[failed]\n");
  serial_println!("Error: {}\n", info);
  exit_qemu(QemuExitCode::Failed);
  loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  test_main();
  loop {}
}
