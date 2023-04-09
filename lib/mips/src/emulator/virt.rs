#[non_exhaustive]
pub struct MemMap;

#[rustfmt::skip]
impl MemMap {
  pub const LOWMEM:     MemRegion = MemRegion { base: 0x0000_0000, size: 0x1000_0000 };
  pub const PM:         MemRegion = MemRegion { base: 0x1008_0000, size: 0x0000_0100 };
  pub const FW_CFG:     MemRegion = MemRegion { base: 0x1008_0100, size: 0x0000_0100 };
  pub const RTC:        MemRegion = MemRegion { base: 0x1008_1000, size: 0x0000_1000 };
  pub const PCIE_PIO:   MemRegion = MemRegion { base: 0x1800_0000, size: 0x0008_0000 };
  pub const PCIE_ECAM:  MemRegion = MemRegion { base: 0x1a00_0000, size: 0x0200_0000 };
  pub const BIOS_ROM:   MemRegion = MemRegion { base: 0x1fc0_0000, size: 0x0020_0000 };
  pub const UART:       MemRegion = MemRegion { base: 0x1fe0_01e0, size: 0x0000_0008 };
  pub const LIOINTC:    MemRegion = MemRegion { base: 0x3ff0_1400, size: 0x0000_0064 };
  pub const PCIE_MMIO:  MemRegion = MemRegion { base: 0x4000_0000, size: 0x4000_0000 };
  pub const HIGHMEM:    MemRegion = MemRegion { base: 0x8000_0000, size: 0x0000_0000 }; /* Variable */
}

#[derive(Debug, Clone, Copy)]
pub struct MemRegion {
  pub base: u32,
  pub size: u32,
}
