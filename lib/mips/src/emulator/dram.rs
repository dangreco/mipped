use crate::interrupt_exception;

use super::{interrupt::Result, virt::MemMap};

pub const DRAM_SIZE: u64 = 1024 * 1024 * 128; // 128 MiB

#[derive(Debug, Clone)]
pub struct Dram {
  pub dram: Vec<u8>,
}

impl Default for Dram {
  fn default() -> Self {
    Dram {
      dram: vec![0; DRAM_SIZE as usize],
    }
  }
}

impl Dram {
  pub fn new(code: Vec<u8>) -> Dram {
    let mut dram = vec![0; DRAM_SIZE as usize];
    dram.splice(..code.len(), code.iter().cloned());

    Self { dram }
  }

  pub fn size(&self) -> u32 {
    self.dram.len() as u32
  }

  pub fn load(&self, addr: u32, size: u32) -> Result<u32> {
    match size {
      8 => Ok(self.load8(addr)),
      16 => Ok(self.load16(addr)),
      32 => Ok(self.load32(addr)),
      _ => interrupt_exception!(DBUS(format!("Cannot load value of {} bytes", size))),
    }
  }

  pub fn store(&mut self, addr: u32, size: u32, value: u32) -> Result<()> {
    match size {
      8 => Ok(self.store8(addr, value)),
      16 => Ok(self.store16(addr, value)),
      32 => Ok(self.store32(addr, value)),
      _ => interrupt_exception!(DBUS(format!("Cannot store value of {} bytes", size))),
    }
  }

  #[inline]
  fn get_index(addr: u32) -> usize {
    (addr - MemMap::HIGHMEM.base) as usize
  }

  fn load8(&self, addr: u32) -> u32 {
    let index = Dram::get_index(addr);
    return self.dram[index] as u32;
  }

  fn load16(&self, addr: u32) -> u32 {
    let index = Dram::get_index(addr);
    return (self.dram[index] as u32) | ((self.dram[index + 1] as u32) << 8);
  }

  fn load32(&self, addr: u32) -> u32 {
    let index = Dram::get_index(addr);
    return (self.dram[index] as u32)
      | ((self.dram[index + 1] as u32) << 8)
      | ((self.dram[index + 2] as u32) << 16)
      | ((self.dram[index + 3] as u32) << 24);
  }

  fn store8(&mut self, addr: u32, value: u32) {
    let index = Dram::get_index(addr);
    self.dram[index] = (value & 0xff) as u8;
  }

  fn store16(&mut self, addr: u32, value: u32) {
    let index = Dram::get_index(addr);
    self.dram[index] = (value & 0xff) as u8;
    self.dram[index + 1] = ((value >> 8) & 0xff) as u8;
  }

  fn store32(&mut self, addr: u32, value: u32) {
    let index = Dram::get_index(addr);
    self.dram[index] = (value & 0xff) as u8;
    self.dram[index + 1] = ((value >> 8) & 0xff) as u8;
    self.dram[index + 2] = ((value >> 16) & 0xff) as u8;
    self.dram[index + 3] = ((value >> 24) & 0xff) as u8;
  }
}
