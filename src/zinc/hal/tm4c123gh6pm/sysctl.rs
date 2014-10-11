//! Low level system control (PLL, clock gating, ...)

/// SysCtl base address
static BASE: u32 = 0x400FE000;

pub mod periph {
  //! peripheral system control

  use core::iter::range;
  use hal::tm4c123gh6pm::io::Reg;

  /// Run mode clock gating control offset
  static RMCGC_OFFSET: u32 = 0x600;

  /// Sysctl can reset/clock gate each module, as well as set various sleep and
  /// deep-sleep mode behaviour.
  pub struct Periph {
    /// Hardware register offset for this peripheral class within a system
    /// control block.
    class: u8,
    /// Bit offset within the class register for this particular peripheral
    id   : u8,
  }

  impl Periph {

    /// Retrieve the clock gating control register address
    fn clock_gating_reg(&self) -> Reg {
      Reg::new(super::BASE + RMCGC_OFFSET + (self.class as u32))
    }

    /// Enable a peripheral
    pub fn enable(&self) {
      let cgr = self.clock_gating_reg();

      // Enable peripheral clock
      cgr.bitband_write(self.id, true);

      // The manual says we have to wait for 3 clock cycles before we can access
      // the peripheral. Waiting for 3 NOPs don't seem to be enough on my board,
      // maybe because we also have to take the bus write time into account or
      // the CPU is more clever than I think. Anyway, looping 5 times seems to
      // work
      for _ in range(0u, 5) {
        unsafe {
          asm!("nop" :::: "volatile");
        }
      }
    }

    /// Check if the peripheral is enabled. If not, enable it.
    pub fn ensure_enabled(&self) {
      let cgr = self.clock_gating_reg();

      if (cgr.read32() & (1 << (self.id as uint))) == 0 {
        self.enable();
      }
    }
  }

  pub mod gpio {
    //! GPIO system control peripherals. Split into ports of 8 GPIO each.

    static CLASS: u8 = 0x8;

    pub static PORT_A: super::Periph = super::Periph { class: CLASS, id: 0 };
    pub static PORT_B: super::Periph = super::Periph { class: CLASS, id: 1 };
    pub static PORT_C: super::Periph = super::Periph { class: CLASS, id: 2 };
    pub static PORT_D: super::Periph = super::Periph { class: CLASS, id: 3 };
    pub static PORT_E: super::Periph = super::Periph { class: CLASS, id: 4 };
    pub static PORT_F: super::Periph = super::Periph { class: CLASS, id: 5 };
  }

}
