#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(k::test_runner)]
#![reexport_test_harness_main = "test_main"]

use k::test_should_panic;
use volatile::Volatile;

test_should_panic!(stack_overflow, || {
  stack_overflow();
});

#[allow(unconditional_recursion)]
fn stack_overflow() {
  stack_overflow();
  Volatile::new(0).read();
}
