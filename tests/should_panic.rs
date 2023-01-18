#![no_std]
#![no_main]

use k::test_should_panic;

test_should_panic!(should_panic, || {
  panic!("This test should panic");
});
