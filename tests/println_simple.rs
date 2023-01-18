#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(k::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use k::{
  println, test,
  vga::{BUFFER_HEIGHT, WRITER},
};

test!(println_simple, || {
  let mut passed = true;

  let s = "Lorem ipsum dolor sit amet";
  println!("{}", s);
  for (i, c) in s.chars().enumerate() {
    let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
    passed = passed && (char::from(screen_char.ascii_character) == c);
  }

  passed
});
