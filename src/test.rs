#[cfg(test)]
use crate::println;

#[cfg(test)]
use core::fmt::Debug;

#[cfg(test)]
pub fn assert<T: PartialEq + Debug>(left: T, right: T) {
  if left == right {
    println!("[ok]");
  } else {
    println!("[failed] {:?} != {:?}", left, right);
  }
}
