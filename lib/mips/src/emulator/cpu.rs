use crate::{interrupt_exception, interrupt_software};

use super::{arch::Register, bus::Bus, interrupt::*, virt::MemMap};

#[allow(dead_code)]
fn sign_ext(value: u32, from: usize) -> i32 {
  let sign = value & (1 << (from - 1));
  if sign != 0 {
    (value | (0xffff_ffff << (32 - from))) as i32
  } else {
    value as i32
  }
}

#[derive(Debug, Clone)]
pub struct Cpu {
  pub regs: [u32; 32],
  pub pc: usize,
  pub tmp: u32,
  pub hi: u32,
  pub lo: u32,
  pub bus: Bus,
}

impl Default for Cpu {
  fn default() -> Self {
    Self {
      regs: [0u32; 32],
      pc: 0,
      tmp: 0,
      hi: 0,
      lo: 0,
      bus: Bus::default(),
    }
  }
}

impl Cpu {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn load(&mut self, code: Vec<u8>) -> Result<()> {
    self.bus.dram_splice(0, code)
  }

  pub fn step(&mut self) -> Result<bool> {
    if self.pc < self.bus.dram.size() as usize {
      let inst = self.fetch()?;
      if inst == 0 {
        Ok(true)
      } else {
        self.pc += 4;
        self.execute(inst)?;
        Ok(false)
      }
    } else {
      Ok(true)
    }
  }

  fn fetch(&self) -> Result<u32> {
    let addr = (self.pc as u32) + MemMap::HIGHMEM.base;
    self.bus.load(addr, 32)
  }

  fn execute(&mut self, inst: u32) -> Result<()> {
    let r = &mut self.regs;
    let opcode = (inst >> 26) & 0x3f;
    let rs = ((inst >> 21) & 0x1f) as usize;
    let rt = ((inst >> 16) & 0x1f) as usize;
    let rd = ((inst >> 11) & 0x1f) as usize;

    match opcode {
      /* ----- R-Type Instructions ----- */
      0x00 => {
        let funct = inst & 0x3f;
        let shamt = ((inst >> 6) & 0x1f) as usize;

        match funct {
          /* SLL $rd, $rt, shamt */
          0x00 => r[rd] = r[rt] << shamt,

          /* SRL $rd, $rt, shamt */
          0x02 => r[rd] = r[rt] >> shamt,

          /* SRA $rd, $rt, shamt */
          0x03 => r[rd] = ((r[rt] as i32) >> shamt) as u32,

          /* SLLV $rd, $rt, $rs*/
          0x04 => r[rd] = r[rt] << r[rs],

          /* SRLV $rd, $rt, $rs */
          0x06 => r[rd] = r[rt] >> r[rs],

          /* SRAV $rd, $rt, $rs */
          0x07 => r[rd] = ((r[rt] as i32) >> (r[rs] & 0x1f)) as u32,

          /* JR $rs */
          0x08 => {
            let addr = r[rs];
            if addr % 4 != 0 {
              interrupt_exception!(ALIGNMENT(addr))
            }
            self.pc = addr as usize;
          }

          /* JALR $rd, $rs */
          0x09 => {
            if rs == rd {
              interrupt_exception!(UNDEFINED)
            }
            let addr = r[rs];
            if addr % 4 != 0 {
              interrupt_exception!(ALIGNMENT(addr))
            }
            self.tmp = addr;
            r[rd] = (self.pc + 8) as u32;
            self.pc = self.tmp as usize;
          }

          /* SYSCALL */
          0x0C => interrupt_software!(SYSCALL),

          /* MFHI $rd */
          0x10 => r[rd] = self.hi,

          /* MTHI $rs */
          0x11 => self.hi = r[rs],

          /* MFLO $rd */
          0x12 => r[rd] = self.lo,

          /* MTLO $rs */
          0x13 => self.lo = r[rs],

          /* MULT $rs, $rt */
          0x18 => {
            let a = r[rs] as i32 as i64;
            let b = r[rt] as i32 as i64;
            let res = a * b;
            self.hi = (res >> 32) as u32;
            self.lo = res as u32;
          }

          /* MULTU $rs, $rt */
          0x19 => {
            let a = r[rs] as u64;
            let b = r[rt] as u64;
            let res = a * b;
            self.hi = (res >> 32) as u32;
            self.lo = res as u32;
          }

          /* DIV $rs, $rt */
          0x1A => {
            let a = r[rs] as i32 as i64;
            let b = r[rt] as i32 as i64;
            self.lo = (a / b) as u32;
            self.hi = (a % b) as u32
          }

          /* DIVU $rs, $rt */
          0x1B => {
            let a = r[rs] as u64;
            let b = r[rt] as u64;
            self.lo = (a / b) as u32;
            self.hi = (a % b) as u32
          }

          /* ADD $rd, $rs, $rt */
          0x20 => {
            let a = r[rs] as i32;
            let b = r[rt] as i32;
            match a.checked_add(b) {
              Some(res) => r[rd] = res as u32,
              None => interrupt_exception!(OVF),
            }
          }

          /* ADDU $rd, $rs, $rt */
          0x21 => r[rd] = r[rs] + r[rt],

          /* SUB $rd, $rs, $rt */
          0x22 => {
            let a = r[rs] as i32;
            let b = r[rt] as i32;
            match a.checked_sub(b) {
              Some(res) => r[rd] = res as u32,
              None => interrupt_exception!(OVF),
            }
          }

          /* SUBU $rd, $rs, $rt */
          0x23 => r[rd] = r[rs] - r[rt],

          /* AND $rd, $rs, $rt */
          0x24 => r[rd] = r[rs] & r[rt],

          /* OR $rd, $rs, $rt */
          0x25 => r[rd] = r[rs] | r[rt],

          /* XOR $rd, $rs, $rt */
          0x26 => r[rd] = r[rs] ^ r[rt],

          /* NOR $rd, $rs, $rt */
          0x27 => r[rd] = !(r[rs] | r[rt]),

          /* SLT $rd, $rs, $rt */
          0x2A => {
            let a = r[rs] as i32;
            let b = r[rt] as i32;
            r[rd] = if a < b { 1 } else { 0 }
          }

          /* SLTU $rd, $rs, $rt */
          0x2B => r[rd] = if r[rs] < r[rt] { 1 } else { 0 },

          _ => interrupt_exception!(UNSUPPORTED(inst)),
        }
      }

      /* ----- J-Type Instructions ----- */

      /* J address */
      0x02 => {
        let target = inst & 0x3ffffff;
        self.pc = (((self.pc + 4) as u32 & 0xf0000000) | (target << 2)) as usize;
      }

      /* JAL address */
      0x03 => {
        r[Register::RA] = (self.pc + 8) as u32;
        let target = inst & 0x3ffffff;
        self.pc = (((self.pc + 4) as u32 & 0xf0000000) | (target << 2)) as usize;
      }

      /* ----- I-Type Instructions ----- */

      /* BEQ $rs, $rt, imm */
      0x04 => {
        if r[rs] == r[rt] {
          let imm = sign_ext((inst & 0xffff) << 2, 18);
          self.pc = ((self.pc as i32 + 4) + imm) as usize;
        }
      }

      /* BNE $rs, $rt, imm */
      0x05 => {
        if r[rs] != r[rt] {
          let imm = sign_ext((inst & 0xffff) << 2, 18);
          self.pc = ((self.pc as i32 + 4) + imm) as usize;
        }
      }

      /* BLEZ $rs, imm */
      0x06 => {
        let a = r[rs] as i32;
        if a <= 0 {
          let imm = sign_ext((inst & 0xffff) << 2, 18);
          self.pc = ((self.pc as i32 + 4) + imm) as usize;
        }
      }

      /* BGTZ $rs, imm */
      0x07 => {
        let a = r[rs] as i32;
        if a > 0 {
          let imm = sign_ext((inst & 0xffff) << 2, 18);
          self.pc = ((self.pc as i32 + 4) + imm) as usize;
        }
      }

      /* ADDI $rt, $rs, imm */
      0x08 => {
        let a = r[rs] as i32;
        let imm = sign_ext(inst & 0xffff, 16);
        r[rt] = (a + imm) as u32;
      }

      /* ADDIU $rt, $rs, imm */
      0x09 => {
        let a = r[rs] as i32;
        let imm = sign_ext(inst & 0xffff, 16);
        match a.checked_add(imm) {
          Some(result) => r[rt] = result as u32,
          None => interrupt_exception!(OVF),
        }
      }

      /* SLTI $rt, $rs, imm */
      0x0A => {
        let a = r[rs] as i32;
        let imm = sign_ext(inst & 0xffff, 16);
        r[rt] = if a < imm { 1 } else { 0 };
      }

      /* SLTIU $rt, $rs, imm */
      0x0B => {
        let a = r[rs];
        let imm = inst & 0xffff;
        r[rt] = if a < imm { 1 } else { 0 };
      }

      /* ANDI $rt, $rs, imm */
      0x0C => r[rt] = r[rs] & (inst & 0xffff),

      /* ORI $rt, $rs, imm */
      0x0D => r[rt] = r[rs] | (inst & 0xffff),

      /* XORI $rt, $rs, imm */
      0x0E => r[rt] = r[rs] ^ (inst & 0xffff),

      /* LUI, $rt, imm */
      0x0F => r[rt] = (inst & 0xffff) << 16,

      /* LB $rt, imm($rs) */
      0x20 => {
        let addr = ((r[rs] as i32) + sign_ext(inst & 0xffff, 16)) as u32;
        let byte = self.bus.load(addr, 8)?;
        r[rt] = sign_ext(byte, 8) as u32;
      }

      /* LH $rt, imm($rs) */
      0x21 => {
        let addr = ((r[rs] as i32) + sign_ext(inst & 0xffff, 16)) as u32;
        let half = self.bus.load(addr, 16)?;
        r[rt] = sign_ext(half, 16) as u32;
      }

      /* LW $rt, imm($rs) */
      0x22 => {
        let addr = ((r[rs] as i32) + sign_ext(inst & 0xffff, 16)) as u32;
        let word = self.bus.load(addr, 32)?;
        r[rt] = word;
      }

      /* LBU $rt, imm($rs) */
      0x24 => {
        let addr = ((r[rs] as i32) + sign_ext(inst & 0xffff, 16)) as u32;
        let byte = self.bus.load(addr, 8)?;
        r[rt] = byte & 0xff;
      }

      /* LHU $rt, imm($rs) */
      0x25 => {
        let addr = ((r[rs] as i32) + sign_ext(inst & 0xffff, 16)) as u32;
        let half = self.bus.load(addr, 16)?;
        r[rt] = half & 0xffff;
      }

      /* SB $rt, imm($rs) */
      0x28 => {
        let addr = ((r[rs] as i32) + sign_ext(inst & 0xffff, 16)) as u32;
        let byte = r[rt] & 0xff;
        self.bus.store(addr, 8, byte)?;
      }

      /* SH $rt, imm($rs) */
      0x29 => {
        let addr = ((r[rs] as i32) + sign_ext(inst & 0xffff, 16)) as u32;
        let half = r[rt] & 0xffff;
        self.bus.store(addr, 16, half)?;
      }

      /* SW $rt, imm($rs) */
      0x2B => {
        let addr = ((r[rs] as i32) + sign_ext(inst & 0xffff, 16)) as u32;
        let word = r[rt];
        self.bus.store(addr, 32, word)?;
      }

      _ => interrupt_exception!(UNSUPPORTED(inst)),
    }

    Ok(())
  }
}
