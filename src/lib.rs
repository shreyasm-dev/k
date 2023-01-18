#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupt;
pub mod qemu;
pub mod serial;
pub mod test;
pub mod util;
pub mod vga;

use core::panic::PanicInfo;
use gdt::init_gdt;
use interrupt::{init_idt, PICS};
use qemu::{exit_qemu, QemuExitCode};
use util::{failed, ok, running};
use x86_64::instructions;

pub fn init() {
  init_gdt();
  init_idt();
  unsafe { PICS.lock().initialize() };
  instructions::interrupts::enable();
}

pub fn test_runner(tests: &[&dyn Fn() -> (&'static str, fn() -> bool)]) {
  let results = tests.iter().map(|test| {
    let test = test();
    running(test.0);
    (test.0, test.1())
  });

  for (name, passed) in results {
    if passed {
      ok(name);
    } else {
      failed(name);
    }
  }

  exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
  test_panic_handler(info)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
  serial_println!("{}", info);
  exit_qemu(QemuExitCode::Failed);
  halt();
}

pub fn halt() -> ! {
  loop {
    instructions::hlt();
  }
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  init();
  test_main();
  halt();
}
