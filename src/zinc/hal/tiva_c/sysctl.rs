//! Low level system control (PLL, clock gating, ...)

pub mod clock {
  //! Clock tree configuration

  use core::option::{Option, None};

  /// Clock sources available on the system
  pub enum ClockSource {
    /// The Precision Internal Oscillator @16MHz
    PIOSC,
    /// PIOSC divided by 4, resulting in a 4MHz source.
    PIOSC_4MHz,
    /// The Main Oscillator, external crystal/oscillator on OSC pins.
    /// The possible frequencies are listed in MOSCSource.
    MOSC,
    /// The Low Frequency Internal Oscillator @30kHz
    LFIOSC,
    /// The Hibernation Oscillator, external crystal/oscillator on XOSC pins.
    /// Frequency should always be 32.768kHz
    HOSC,
  }

  /// The chip supports a finite list of crystal frequencies for the MOSC, each
  /// having its own ID used to configure the PLL to output 400MHz.
  #[allow(missing_doc)]
  pub enum MOSCFreq {
    X5_0MHz    = 0x09,
    X5_12MHz   = 0x0A,
    X6_0MHz    = 0x0B,
    X6_144MHz  = 0x0C,
    X7_3728MHz = 0x0D,
    X8_0MHz    = 0x0E,
    X8_192MHz  = 0x0F,
    X10_0MHz   = 0x10,
    X12_0MHz   = 0x11,
    X12_288MHz = 0x12,
    X13_56MHz  = 0x13,
    X14_318MHz = 0x14,
    X16_0MHz   = 0x15,
    X16_384MHz = 0x16,
    X18MHz     = 0x17,
    X20MHz     = 0x18,
    X24MHz     = 0x19,
  }

  /// Configure the System Clock by setting the clock source and divisors.
  fn sysclk_configure(source:      ClockSource,
                      mosc_source: Option<MOSCFreq>,
                      pll_div:     Option<uint>) {
    
  }
}

pub mod periph {
  //! peripheral system control

  use core::iter::range;
  use hal::tiva_c::io::Reg;

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

    const CLASS: u8 = 0x8;

    pub const PORT_A: super::Periph = super::Periph { class: CLASS, id: 0 };
    pub const PORT_B: super::Periph = super::Periph { class: CLASS, id: 1 };
    pub const PORT_C: super::Periph = super::Periph { class: CLASS, id: 2 };
    pub const PORT_D: super::Periph = super::Periph { class: CLASS, id: 3 };
    pub const PORT_E: super::Periph = super::Periph { class: CLASS, id: 4 };
    pub const PORT_F: super::Periph = super::Periph { class: CLASS, id: 5 };
  }

  pub mod timer {
    //! Timer system control peripherals. Each timer has two independent
    //! counters (A and B).

    const TIMER_CLASS:   u8 = 0x4;
    const TIMER_W_CLASS: u8 = 0x5c;

    pub const TIMER_0: super::Periph = super::Periph { class: TIMER_CLASS,
                                                        id: 0 };
    pub const TIMER_1: super::Periph = super::Periph { class: TIMER_CLASS,
                                                        id: 1 };
    pub const TIMER_2: super::Periph = super::Periph { class: TIMER_CLASS,
                                                        id: 2 };
    pub const TIMER_3: super::Periph = super::Periph { class: TIMER_CLASS,
                                                        id: 3 };
    pub const TIMER_4: super::Periph = super::Periph { class: TIMER_CLASS,
                                                        id: 4 };
    pub const TIMER_5: super::Periph = super::Periph { class: TIMER_CLASS,
                                                        id: 5 };

    pub const TIMER_W_0: super::Periph = super::Periph { class: TIMER_W_CLASS,
                                                          id: 0 };
    pub const TIMER_W_1: super::Periph = super::Periph { class: TIMER_W_CLASS,
                                                          id: 1 };
    pub const TIMER_W_2: super::Periph = super::Periph { class: TIMER_W_CLASS,
                                                          id: 2 };
    pub const TIMER_W_3: super::Periph = super::Periph { class: TIMER_W_CLASS,
                                                          id: 3 };
    pub const TIMER_W_4: super::Periph = super::Periph { class: TIMER_W_CLASS,
                                                          id: 4 };
    pub const TIMER_W_5: super::Periph = super::Periph { class: TIMER_W_CLASS,
                                                          id: 5 };
  }

  pub mod uart {
    //! UART peripherals instances
    const CLASS: u8 = 0x18;

    pub const UART_0: super::Periph = super::Periph { class: CLASS, id: 0 };
    pub const UART_1: super::Periph = super::Periph { class: CLASS, id: 1 };
    pub const UART_2: super::Periph = super::Periph { class: CLASS, id: 2 };
    pub const UART_3: super::Periph = super::Periph { class: CLASS, id: 3 };
    pub const UART_4: super::Periph = super::Periph { class: CLASS, id: 4 };
    pub const UART_5: super::Periph = super::Periph { class: CLASS, id: 5 };
    pub const UART_6: super::Periph = super::Periph { class: CLASS, id: 6 };
    pub const UART_7: super::Periph = super::Periph { class: CLASS, id: 7 };
  }

  /// Run mode clock gating control offset
  const RMCGC_OFFSET: u32 = 0x600;
}

/// SysCtl base address
const BASE: u32 = 0x400FE000;
