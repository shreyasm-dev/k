#[cfg(test)]
use crate::serial_println;

#[cfg(test)]
pub fn assert(condition: bool) -> bool {
  if condition {
    serial_println!("[ok]");
  } else {
    serial_println!("[failed]");
  }

  condition
}

#[macro_export]
macro_rules! test {
  ($name:ident, $test:expr) => {
    #[test_case]
    fn $name() -> bool {
      serial_print!("{} ... ", stringify!($name));
      $test()
    }
  };
}
