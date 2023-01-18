#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(k::test_runner)]
#![reexport_test_harness_main = "test_main"]

use k::test_should_fail;

test_should_fail!(should_panic, || {
  panic!("This test should panic");
});
