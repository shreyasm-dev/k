#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

use vga::Writer;

static TEXT: &'static str = "Hello, buffer!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  let mut writer = Writer {
    column_position: 0,
    color_code: vga::ColorCode::new(vga::Color::Yellow, vga::Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut vga::Buffer) },
  };

  writer.write_string(TEXT);

  loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}
