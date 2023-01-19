use crate::{
  print, println,
  vga::{backspace, clear_screen, BUFFER_WIDTH},
};
use core::str::from_utf8_unchecked;
use lazy_static::lazy_static;
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use spin::lock_api::Mutex;
use x86_64::instructions::interrupts;

const INPUT_WIDTH: usize = BUFFER_WIDTH - 1;
static mut PROMPT: char = '>';

pub fn prompt() {
  unsafe { print!("{} ", PROMPT); }
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
    match str {
      "help" => {
        println!("
Available commands:
  help - Show this message
  about - Show information about the OS
  clear - Clear the screen
  rand <seed: u64> - Generate a cryptographically insecure random number using <seed> (if seed is not a valid u64, 0 is used)
  panic - Panic the kernel
  echo <text> - Print <text> to the screen
  setprompt <c: char> - Set the prompt to <c> (if c is longer than 1 character, the first character is used)");
      }
      "about" => println!("\nSimple operating system written in Rust, developed by shreyasm-dev"),
      "clear" => {
        interrupts::without_interrupts(|| {
          clear_screen();
        });

        println!();
      }
      "panic" => {
        println!();
        panic!("Panic from shell")
      }
      "rand" => println!("\nMissing seed"),
      "setprompt" => println!("\nMissing prompt character"),
      "echo" => println!("\n"),
      _ => {
        if str.starts_with("echo ") {
          println!("\n{}", &str[5..]);
        } else if str.starts_with("rand ") {
          let seed = str[5..].parse::<u64>().unwrap_or(0);
          println!("\n{}", XorShiftRng::seed_from_u64(seed).next_u64());
        } else if str.starts_with("setprompt ") {
          unsafe {
            PROMPT = str.chars().nth(10).unwrap_or('>');
          }

          println!();
        } else {
          println!("\nUnknown command, type 'help' for a list of available commands");
        }
      }
    }

    prompt();
  } else if shell.add_char(key) {
    print!("{}", key);
  }

  unsafe {
    SHELL.force_unlock();
  }
}
