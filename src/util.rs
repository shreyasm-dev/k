use crate::serial_println;

pub fn running(text: &'static str) {
  serial_println!("\x1b[1;37;44m[running]\x1b[0m {}", text);
}

pub fn ok(text: &'static str) {
  serial_println!("\x1b[1;30;47m[ok]\x1b[0m {}", text);
}

pub fn failed(text: &'static str) {
  serial_println!("\x1b[1;37;41m[failed]\x1b[0m {}", text);
}
