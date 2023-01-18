#[macro_export]
macro_rules! test {
  ($name:ident, $test:expr) => {
    #[test_case]
    fn $name() -> (&'static str, fn() -> bool) {
      (stringify!($name), $test)
    }

    #[no_mangle]
    pub extern "C" fn _start() -> ! {
      #[cfg(test)]
      test_main();
      loop {}
    }
  };
}
