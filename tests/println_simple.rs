#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(k::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::fmt::Write;

#[cfg(test)]
use k::{
  test,
  vga::{BUFFER_HEIGHT, WRITER},
};
use x86_64::instructions::interrupts;

test!(println_simple, || {
  let mut passed = true;

  let s = "Lorem ipsum dolor sit amet";
  interrupts::without_interrupts(|| {
    let mut writer = WRITER.lock();
    writeln!(writer, "\n{}", s).unwrap();
    for (i, c) in s.chars().enumerate() {
      let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
      passed = passed && (char::from(screen_char.ascii_character) == c);
    }
  });

  passed
});
