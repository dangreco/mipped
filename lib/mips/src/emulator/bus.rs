use crate::interrupt_exception;

use super::{
  dram::Dram,
  interrupt::Result,
  virt::{MemMap, MemRegion},
};

#[derive(Debug, Clone)]
pub struct Bus {
  pub dram: Dram,
}

impl Default for Bus {
  fn default() -> Self {
    Self {
      dram: Dram::default(),
    }
  }
}

impl Bus {
  pub fn load(&self, addr: u32, size: u32) -> Result<u32> {
    let valid = Bus::ensure_valid_address(
      addr,
      MemRegion {
        base: MemMap::HIGHMEM.base,
        size: self.dram.size(),
      },
    );

    match valid {
      Ok(_) => self.dram.load(addr, size),
      Err(_) => interrupt_exception!(ADDRL(addr)),
    }
  }

  pub fn store(&mut self, addr: u32, size: u32, value: u32) -> Result<()> {
    let valid = Bus::ensure_valid_address(
      addr,
      MemRegion {
        base: MemMap::HIGHMEM.base,
        size: self.dram.size(),
      },
    );

    match valid {
      Ok(_) => self.store(addr, size, value),
      Err(_) => interrupt_exception!(ADDRS(addr)),
    }
  }

  pub fn dram_splice(&mut self, offset: u32, code: Vec<u8>) -> Result<()> {
    Bus::ensure_valid_address(
      MemMap::HIGHMEM.base + offset,
      MemRegion {
        base: MemMap::HIGHMEM.base,
        size: self.dram.size(),
      },
    )?;
    Bus::ensure_valid_address(
      MemMap::HIGHMEM.base + offset + (code.len() as u32),
      MemRegion {
        base: MemMap::HIGHMEM.base,
        size: self.dram.size(),
      },
    )?;

    let start = offset as usize;
    let end = start + code.len();

    self.dram.dram.splice(start..end, code.iter().cloned());

    Ok(())
  }

  fn ensure_valid_address(addr: u32, region: MemRegion) -> Result<()> {
    let start = region.base;
    let end = start + region.size;

    if addr >= start && addr < end {
      return Ok(());
    }

    interrupt_exception!(DBUS(format!("Invalid memory address: {:#010X}", addr)))
  }
}
