#![no_std]
#![no_main]

use k::test_should_fail;

test_should_fail!(should_panic, || {
  panic!("This test should panic");
});
