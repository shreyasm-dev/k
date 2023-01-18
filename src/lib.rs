#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod interrupt;
pub mod qemu;
pub mod serial;
pub mod test;
pub mod vga;

use core::panic::PanicInfo;
use interrupt::init_idt;
use qemu::{exit_qemu, QemuExitCode};

pub fn init() {
  init_idt();
}

pub fn test_runner(tests: &[&dyn Fn() -> (&'static str, fn() -> bool)]) {
  let results = tests.iter().map(|test| {
    let test = test();
    serial_println!("\x1b[1;37;44m[running]\x1b[0m {}", test.0);
    (test.0, test.1())
  });

  for (name, passed) in results {
    if passed {
      serial_println!("\x1b[1;30;47m[ok]\x1b[0m {}", name);
    } else {
      serial_println!("\x1b[1;37;41m[failed]\x1b[0m {}", name);
    }
  }

  exit_qemu(QemuExitCode::Success);
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  exit_qemu(QemuExitCode::Failed);
  loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  init();
  test_main();
  loop {}
}
