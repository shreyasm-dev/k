#[cfg(test)]
use crate::serial_println;

#[cfg(test)]
use core::fmt::Debug;

#[cfg(test)]
pub fn assert(condition: bool) -> bool {
  if condition {
    serial_println!("[ok]");
  } else {
    serial_println!("[failed]");
  }

  condition
}

#[cfg(test)]
pub fn assert_eq<T: PartialEq + Debug>(left: T, right: T) -> bool {
  if left == right {
    serial_println!("[ok]");
  } else {
    serial_println!("[failed] {:?} != {:?}", left, right);
  }

  left == right
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
