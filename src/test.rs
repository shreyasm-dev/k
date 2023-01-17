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
