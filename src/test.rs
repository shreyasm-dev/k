#[cfg(test)]
use crate::println;

#[cfg(test)]
use core::fmt::Debug;

#[cfg(test)]
pub fn assert(condition: bool) -> bool {
  if condition {
    println!("[ok]");
  } else {
    println!("[failed]");
  }

  condition
}

#[cfg(test)]
pub fn assert_eq<T: PartialEq + Debug>(left: T, right: T) -> bool {
  if left == right {
    println!("[ok]");
  } else {
    println!("[failed] {:?} != {:?}", left, right);
  }

  left == right
}
