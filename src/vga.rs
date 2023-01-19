use core::fmt::{Arguments, Result, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::{interrupts, port::Port};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
  Black = 0,
  Blue = 1,
  Green = 2,
  Cyan = 3,
  Red = 4,
  Magenta = 5,
  Brown = 6,
  LightGray = 7,
  DarkGray = 8,
  LightBlue = 9,
  LightGreen = 10,
  LightCyan = 11,
  LightRed = 12,
  Pink = 13,
  Yellow = 14,
  White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(pub u8);

impl ColorCode {
  pub fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
  pub ascii_character: u8,
  pub color_code: ColorCode,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
  pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
  pub column_position: usize,
  pub color_code: ColorCode,
  pub buffer: &'static mut Buffer,
}

impl Writer {
  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        let color_code = self.color_code;
        self.buffer.chars[row][col].write(ScreenChar {
          ascii_character: byte,
          color_code,
        });
        self.column_position += 1;
      }
    }

    unsafe { move_cursor(BUFFER_HEIGHT - 1, self.column_position); }
  }

  fn new_line(&mut self) {
    for row in 1..BUFFER_HEIGHT {
      for col in 0..BUFFER_WIDTH {
        let character = self.buffer.chars[row][col].read();
        self.buffer.chars[row - 1][col].write(character);
      }
    }
    self.clear_row(BUFFER_HEIGHT - 1);
    self.column_position = 0;
  }

  fn clear_row(&mut self, row: usize) {
    let blank = ScreenChar {
      ascii_character: b' ',
      color_code: self.color_code,
    };
    for col in 0..BUFFER_WIDTH {
      self.buffer.chars[row][col].write(blank);
    }
  }

  pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        _ => self.write_byte(0xfe),
      }
    }
  }
}

impl Write for Writer {
  fn write_str(&mut self, s: &str) -> Result {
    self.write_string(s);
    Ok(())
  }
}

lazy_static! {
  pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  });
}

#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
  () => ($crate::print!("\n"));
  ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn backspace() {
  interrupts::without_interrupts(|| {
    let mut writer = WRITER.lock();
    if writer.column_position > 0 {
      writer.column_position -= 1;
      writer.write_byte(b' ');
      writer.column_position -= 1;
    }
  });
}

pub fn clear_screen() {
  interrupts::without_interrupts(|| {
    let mut writer = WRITER.lock();
    for i in 0..BUFFER_HEIGHT {
      writer.clear_row(i);
    }

    writer.column_position = BUFFER_WIDTH - 1;
  });
}

pub unsafe fn move_cursor(row: usize, col: usize) {
  let position = (row * BUFFER_WIDTH + col) as u16;
  let mut port_low = Port::new(0x3d4);
  let mut port_high = Port::new(0x3d5);

  port_low.write(0x0f as u8);
  port_high.write((position & 0xff) as u8);
  port_low.write(0x0e as u8);
  port_high.write(((position >> 8) & 0xff) as u8);
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
  interrupts::without_interrupts(|| {
    WRITER.lock().write_fmt(args).unwrap();
  });
}
