use core::str::from_utf8_unchecked;

use crate::{
  print, println,
  vga::{backspace, clear_screen, BUFFER_WIDTH},
};
use lazy_static::lazy_static;
use spin::lock_api::Mutex;
use x86_64::instructions::interrupts;

const PROMPT: &'static str = "> ";
const INPUT_WIDTH: usize = BUFFER_WIDTH - PROMPT.len();

pub fn prompt() {
  print!("{}", PROMPT);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Shell {
  len: usize,
  buffer: [char; INPUT_WIDTH],
}

impl Shell {
  pub fn new() -> Shell {
    Shell {
      len: 0,
      buffer: [' '; INPUT_WIDTH],
    }
  }

  pub fn add_char(&mut self, c: char) -> bool {
    if self.len < INPUT_WIDTH {
      self.buffer[self.len] = c;
      self.len += 1;
      return true;
    }

    false
  }

  pub fn backspace(&mut self) -> bool {
    if self.len > 0 {
      self.len -= 1;
      self.buffer[self.len] = ' ';
      return true;
    }

    false
  }

  pub fn clear(&mut self) {
    self.len = 0;
    self.buffer = [' '; INPUT_WIDTH];
  }
}

lazy_static! {
  static ref SHELL: Mutex<Shell> = Mutex::new(Shell::new());
}

pub fn on_keydown(key: char) {
  let mut shell = SHELL.lock();
  if key == '\u{8}' {
    if shell.backspace() {
      backspace();
    }
  } else if key == '\n' {
    let mut bytes = [0u8; INPUT_WIDTH];
    for (i, c) in shell.buffer.iter().enumerate() {
      bytes[i] = *c as u8;
    }

    let str = unsafe { from_utf8_unchecked(&bytes) }.trim();

    shell.clear();

    println!(
      "\n{}",
      match str {
        "help" =>
          "Available commands:
help - Show this message
about - Show information about the OS
clear - Clear the screen
panic - Panic the kernel
echo <text> - Print <text> to the screen",
        "about" => "Simple operating system written in Rust, developed by shreyasm-dev",
        "clear" => {
          interrupts::without_interrupts(|| {
            clear_screen();
          });

          ""
        }
        "panic" => {
          println!();
          panic!("Panic from shell")
        }
        _ => {
          if str.starts_with("echo ") {
            &str[5..]
          } else {
            "Unknown command, type 'help' for a list of available commands"
          }
        }
      }
    );

    prompt();
  } else if shell.add_char(key) {
    print!("{}", key);
  }

  unsafe {
    SHELL.force_unlock();
  }
}
