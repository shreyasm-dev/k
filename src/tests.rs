
#[cfg(test)]
#[allow(unused_imports)]
use crate::{test::assert_eq, serial_print, serial_println, test};

#[test_case]
test!(trivial_assertion, || assert_eq(1, 2));
