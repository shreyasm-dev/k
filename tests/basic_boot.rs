#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(k::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use k::{println, test};

test!(basic_boot, || {
  println!("Hello, world!");
  true
});
