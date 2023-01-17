use crate::println;
use core::fmt::Debug;

#[allow(dead_code)]
pub fn assert<T: PartialEq + Debug>(left: T, right: T) {
  if left == right {
    println!("[ok]");
  } else {
    println!("[failed] {:?} != {:?}", left, right);
  }
}
