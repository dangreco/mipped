use std::{
  fmt::Display,
  io::{Read, Write},
};

use crate::interrupt_software;

use super::{arch::Register, cpu::Cpu, interrupt::*};

pub struct Sys {
  cpu: Cpu,
  running: bool,
  stdout: Box<dyn Write>,
  stdin: Box<dyn Read>,
  exit_code: Option<i32>,
}

impl Sys {
  pub fn new<O, I>(cpu: Cpu, stdout: O, stdin: I) -> Self
  where
    O: 'static + Write,
    I: 'static + Read,
  {
    Sys {
      cpu,
      stdout: Box::new(stdout),
      stdin: Box::new(stdin),
      running: false,
      exit_code: None,
    }
  }

  fn write<S>(&mut self, str: S) -> Result<()>
  where
    S: Display,
  {
    write!(self.stdout, "{}", str)
      .map_err(|e| Interrupt::Software(SoftwareInterrupt::STDOUT(e.to_string())))
  }

  pub fn run(&mut self) -> Result<()> {
    self.running = true;

    while self.running {
      match self.cpu.step() {
        Ok(done) => self.running = !done,
        Err(err) => match err {
          Interrupt::Software(SoftwareInterrupt::SYSCALL) => self.handle_syscall()?,
          _ => return Err(err),
        },
      }
    }

    self.write(format!(
      "\nProcess exited with code {}\n",
      self.exit_code.unwrap_or(0)
    ))?;

    Ok(())
  }

  pub fn handle_syscall(&mut self) -> Result<()> {
    let r = &mut self.cpu.regs;
    match r[Register::V0] {
      /* Print Integer */
      0x01 => {
        let n = r[Register::A0] as i32;
        self.write(format!("{n}"))?;
      }

      /* Print Float */
      0x02 => {}

      /* Print Double */
      0x03 => {}

      /* Print String */
      0x04 => {
        let addr = r[Register::A0];
        let mut byte = 0;
        loop {
          let c = self.cpu.bus.load(addr + byte, 8)? as u8 as char;

          if c == '\0' {
            break;
          }

          self.write(format!("{c}"))?;

          byte += 1;
        }
      }

      /* Read Integer */
      0x05 => {}

      /* Read Float */
      0x06 => {}

      /* Read Double */
      0x07 => {}

      /* Read String */
      0x08 => {}

      /* SBRK (allocate heap memory) */
      0x09 => {}

      /* Exit (terminate execution) */
      0x0A => self.running = false,

      /* Print Character */
      0x0B => {}

      /* Read Character */
      0x0C => {}

      /* Open File */
      0x0D => {}

      /* Read From File */
      0x0E => {}

      /* Write To File */
      0x0F => {}

      /* Close File */
      0x10 => {}

      /* Exit2 (terminate with value) */
      0x11 => {
        self.exit_code = Some(r[Register::A0] as i32);
        self.running = false;
      }

      /* ------ SPIM ------ */

      /* ------ MARS ------ */

      /* Time (system time) */
      0x1E => {}

      /* MIDI out */
      0x1F => {}

      /* Sleep */
      0x20 => {}

      /* MIDI out (synchronous) */
      0x21 => {}

      /* Print integer (in hexadecimal) */
      0x22 => {}

      /* Print integer (in binary) */
      0x23 => {}

      /* Print integer (as unsigned) */
      0x24 => {}

      /* Set seed */
      0x28 => {}

      /* Random integer */
      0x29 => {}

      /* Random integer range */
      0x2A => {}

      /* Random float */
      0x2B => {}

      /* Random double */
      0x2C => {}

      _ => interrupt_software!(UNSUPPORTED(r[Register::V0])),
    }

    Ok(())
  }
}
