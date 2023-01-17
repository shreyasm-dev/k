#[cfg(test)]
#[allow(unused_imports)]
use crate::{
  test::assert,
  vga::{WRITER, BUFFER_HEIGHT},
  println,
  serial_print,
  serial_println,
  test,
};

#[test_case]
test!(println_simple, || {
  let mut passed = true;

  let s = "Lorem ipsum dolor sit amet";
  println!("{}", s);
  for (i, c) in s.chars().enumerate() {
    let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
    passed = passed && (char::from(screen_char.ascii_character) == c);
  }

  assert(passed)
});
