use crate::{
  print, println,
  vga::{backspace, clear_screen, BUFFER_WIDTH},
};
use core::{arch::x86_64::_rdtsc, str::from_utf8_unchecked};
use lazy_static::lazy_static;
use raw_cpuid::CpuId;
use spin::lock_api::Mutex;
use x86_64::instructions::interrupts;

const INPUT_WIDTH: usize = BUFFER_WIDTH - 2;
static mut PROMPT: char = '>';

pub fn prompt() {
  unsafe {
    print!("{} ", PROMPT);
  }
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
    evaluate_command(str);
    prompt();
  } else if shell.add_char(key) {
    print!("{}", key);
  }

  unsafe {
    SHELL.force_unlock();
  }
}

pub fn evaluate_command(str: &str) {
  let (command, args) = str.split_at(str.find(' ').unwrap_or(str.len()));
  let args = if args.starts_with(' ') {
    &args[1..]
  } else {
    args
  };

  println!();

  match command {
    "help" => {
      println!("Available commands:
  help - Show this message
  about - Show information about the OS
  clear - Clear the screen
  panic - Panic the kernel
  echo <text> - Print <text> to the screen
  setprompt <c: char> - Set the prompt to <c> (if c is longer than 1 character, the first character is used)
  cpuid - Get CPU information
  uptime - Get the uptime of the system (in cycles, not seconds)
  memcat <addr: usize> <len: usize> - Print the contents of memory at <addr> with length <len> (hexadecimal is not supported yet)");
    }
    "about" => println!("Simple operating system written in Rust, developed by shreyasm-dev"),
    "clear" => {
      interrupts::without_interrupts(|| {
        clear_screen();
      });

      println!();
    }
    "panic" => {
      panic!("Panic from shell")
    }
    "echo" => {
      println!("{}", args);
    }
    "setprompt" => {
      if args.is_empty() {
        println!("Missing prompt character");
      } else {
        unsafe {
          PROMPT = args.chars().nth(0).unwrap_or('>');
        }
      }
    }
    "cpuid" => {
      let cpuid = CpuId::new();

      let vendor = cpuid.get_vendor_info();
      let vendor = vendor.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");

      let processor = cpuid.get_processor_brand_string();
      let processor = processor.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");

      let tsc = unsafe { _rdtsc() };

      println!(
        "Vendor: {}
Processor: {}
Uptime (cycles): {}",
        vendor, processor, tsc
      );
    }
    "uptime" => {
      let tsc = unsafe { _rdtsc() };
      println!("{}", tsc);
    }
    "memcat" => {
      let mut args_iter = args.split_whitespace();

      let addr = match args_iter.next() {
        Some(arg) => match arg.parse::<usize>() {
          Ok(addr) => addr,
          _ => {
            println!("Invalid address");
            return;
          }
        },
        _ => {
          println!("Missing address");
          return;
        }
      };

      let len = match args_iter.next() {
        Some(arg) => match arg.parse::<usize>() {
          Ok(len) => len,
          _ => {
            println!("Invalid length");
            return;
          }
        },
        _ => {
          println!("Missing length");
          return;
        }
      };

      let mem_output = unsafe { core::slice::from_raw_parts(addr as *const u8, len) };
      println!("{:X?}", mem_output);
    }
    _ => println!("Unknown command, type 'help' for a list of available commands"),
  }
}
