use std::fmt::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Interrupt>;

#[derive(Debug, Error)]
pub enum Interrupt {
  Software(SoftwareInterrupt),
  Hardware(HardwareInterrupt),
  Exception(ExceptionInterrupt),
}

impl Display for Interrupt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Interrupt::Software(software) => software.fmt(f),
      Interrupt::Hardware(hardware) => hardware.fmt(f),
      Interrupt::Exception(exception) => exception.fmt(f),
    }
  }
}

#[derive(Debug, Error)]
pub enum SoftwareInterrupt {
  #[error("syscall")]
  SYSCALL,

  #[error("Error writing to stdout: {0:?}")]
  STDOUT(String),

  #[error("Error reading from stdin: {0:?}")]
  STDIN(String),

  #[error("Unsupported syscall: {0:#4X}")]
  UNSUPPORTED(u32),
}

#[derive(Debug, Error)]
pub enum HardwareInterrupt {}

#[derive(Debug, Error)]
pub enum ExceptionInterrupt {
  #[error("Load from an illegal address: {0:#010X}")]
  ADDRL(u32),

  #[error("Store to an illegal address {0:#010X}")]
  ADDRS(u32),

  #[error("Bus error on instruction fetch: {0:?}")]
  IBUS(String),

  #[error("Bus error on data reference: {0:?}")]
  DBUS(String),

  #[error("Arithmetic overflow")]
  OVF,

  #[error("Value not word-aligned: {0:#010X}")]
  ALIGNMENT(u32),

  #[error("Undefined behavior")]
  UNDEFINED,

  #[error("Unsupported instruction: {0:#010X}")]
  UNSUPPORTED(u32),
}

#[macro_export]
macro_rules! interrupt_software {
  ($x:ident) => {
    return Err(crate::emulator::interrupt::Interrupt::Software(
      crate::emulator::interrupt::SoftwareInterrupt::$x,
    ))
  };
  ($x:ident($e:expr)) => {
    return Err(crate::emulator::interrupt::Interrupt::Software(
      crate::emulator::interrupt::SoftwareInterrupt::$x($e),
    ))
  };
}

#[macro_export]
macro_rules! interrupt_hardware {
  ($x:ident) => {
    return Err(crate::emulator::interrupt::Interrupt::Hardware(
      crate::emulator::interrupt::HardwareInterrupt::$x,
    ))
  };
  ($x:ident($e:expr)) => {
    return Err(crate::emulator::interrupt::Interrupt::Hardware(
      crate::emulator::interrupt::HardwareInterrupt::$x($e),
    ))
  };
}

#[macro_export]
macro_rules! interrupt_exception {
  ($x:ident) => {
    return Err(crate::emulator::interrupt::Interrupt::Exception(
      crate::emulator::interrupt::ExceptionInterrupt::$x,
    ))
  };
  ($x:ident($e:expr)) => {
    return Err(crate::emulator::interrupt::Interrupt::Exception(
      crate::emulator::interrupt::ExceptionInterrupt::$x($e),
    ))
  };
}
