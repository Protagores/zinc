//! Custom register access interface

use util::volatile_cell::VolatileCell;
use core::intrinsics::{volatile_load, volatile_store};

/// Hardware register interface
pub struct Reg {
  /// Register address
  addr: u32,
}

impl Reg {
  /// create a new Reg from a 32bit register address
  pub fn new(addr: u32) -> Reg {
    Reg { addr: addr }
  }

  /// Write to a 32bit register
  #[inline]
  pub fn write32(&self, val: u32) {
    unsafe {
      let r = self.addr as *mut u32;
      volatile_store(r, val);
    }
  }

  /// Read from a 32bit register
  #[inline]
  pub fn read32(&self) -> u32 {
    unsafe {
      let r = self.addr as *const u32;
      volatile_load(r)
    }
  }

  /// Write single bit to a register using hardware bitbanding
  #[inline]
  pub fn bitband_write(&self, bit: u8, set: bool) {
    /* bitband offset */
    let mut bitband = (self.addr & 0xf0000000) | 0x02000000;

    /* register offset */
    bitband |= (self.addr & 0x00fffff) << 5;
    /* bit offset */
    bitband |= (bit as u32) << 2;

    unsafe {
      let r = bitband as *mut u32;
      volatile_store(r, set as u32);
    }
  }
}
