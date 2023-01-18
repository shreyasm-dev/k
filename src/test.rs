#[macro_export]
macro_rules! test {
  ($name:ident, $test:expr) => {
    use k::test_panic_handler;

    #[test_case]
    fn $name() -> (&'static str, fn() -> bool) {
      (stringify!($name), $test)
    }

    #[cfg(test)]
    #[panic_handler]
    pub fn panic(info: &core::panic::PanicInfo) -> ! {
      test_panic_handler(info)
    }

    #[no_mangle]
    pub extern "C" fn _start() -> ! {
      test_main();
      loop {}
    }
  };
}

#[macro_export]
macro_rules! test_should_panic {
  ($name:ident, $test:expr) => {
    use k::{
      qemu::{exit_qemu, QemuExitCode},
      util::{failed, ok, running},
    };

    #[no_mangle]
    pub extern "C" fn _start() -> ! {
      running(stringify!($name));
      $test();
      failed(stringify!($name));
      exit_qemu(QemuExitCode::Failed);
      loop {}
    }

    #[cfg(test)]
    #[panic_handler]
    pub fn panic(_info: &core::panic::PanicInfo) -> ! {
      ok(stringify!($name));
      exit_qemu(QemuExitCode::Success);
      loop {}
    }
  };
}

#[macro_export]
macro_rules! test_should_not_panic {
  ($name:ident, $test:expr) => {
    use k::{
      qemu::{exit_qemu, QemuExitCode},
      util::{failed, ok, running},
    };

    #[no_mangle]
    pub extern "C" fn _start() -> ! {
      running(stringify!($name));
      $test();
      ok(stringify!($name));
      exit_qemu(QemuExitCode::Success);
      loop {}
    }

    #[cfg(test)]
    #[panic_handler]
    pub fn panic(_info: &core::panic::PanicInfo) -> ! {
      failed(stringify!($name));
      exit_qemu(QemuExitCode::Failed);
      loop {}
    }
  };
}

#[macro_export]
macro_rules! test_no_main {
  ($name:ident, $test:expr) => {
    #[test_case]
    fn $name() -> (&'static str, fn() -> bool) {
      (stringify!($name), $test)
    }
  };
}
