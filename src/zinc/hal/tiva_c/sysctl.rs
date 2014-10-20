//! Low level system control (PLL, clock gating, ...)

pub mod clock {
  //! Clock tree configuration

  use core::option::{Option, Some};
  use hal::tiva_c::io::Reg;

  /// Clock sources available on the system. The values are the RCC/RCC2 OSCSRC
  /// field encoding.
  #[deriving(PartialEq)]
  pub enum ClockSource {
    /// The Main Oscillator, external crystal/oscillator on OSC pins.
    /// The possible frequencies are listed in MOSCSource.
    MOSC       = 0,
    /// The Precision Internal Oscillator @16MHz
    PIOSC      = 1,
    /// PIOSC divided by 4, resulting in a 4MHz source.
    PIOSC4MHz = 2,
    /// The Low Frequency Internal Oscillator @30kHz
    LFIOSC     = 3,
    /// The Hibernation Oscillator, external crystal/oscillator on XOSC pins.
    /// Frequency should always be 32.768kHz
    HOSC       = 7,
  }

  /// The chip supports a finite list of crystal frequencies for the MOSC, each
  /// having its own ID used to configure the PLL to output 400MHz.
  #[allow(missing_doc)]
  #[allow(non_camel_case_types)]
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
  pub fn sysclk_configure(source:      ClockSource,
                          mosc_source: Option<MOSCFreq>,
                          use_pll:     bool,
                          div:         Option<uint>) {
    let rcc  = Reg::new(super::BASE + RCC_OFFSET);
    let rcc2 = Reg::new(super::BASE + RCC2_OFFSET);

    let mut rcc_val  = rcc.read32();
    let mut rcc2_val = rcc.read32();

    // Start off by disabling the PLL and dividers, we'll run from the system's
    // clock source directly
    rcc_val  |= 1 << 11; // Bypass PLL
    rcc_val  &= !(1 << 22); // Don't use divider
    rcc2_val |= 1 << 11; // Bypass PLL2

    rcc.write32(rcc_val);
    rcc.write32(rcc2_val);

    // If want to switch to the Main Oscillator but it's disabled, we need to
    // enable it and wait for it to lock
    if source == MOSC && ((rcc_val & 1) != 0) {
      let misc = Reg::new(super::BASE + MISC_OFFSET);

      // Clear any pending MOSC power upinterrupt since we'll have to poll it
      // below
      misc.bitband_write(8, true);

      // Enable MOSC
      rcc_val &= !1;
      rcc.write32(rcc_val);

      let ris = Reg::new(super::BASE + RIS_OFFSET);

      loop {
        if ris.read32() & (1 << 8) != 0 {
          // MOSC locked
          break;
        }
      }

      // XXX: What should we do if the MOSC didn't lock up?
    }

    // Clear previous crystal and source config
    rcc_val &= !((3 << 4) | 0x1f << 6);
    rcc2_val &= !(7 << 4);

    rcc_val |= (source as u32) << 4;
    rcc2_val |= (source as u32) << 4;

    if source == MOSC {
      // Set XTAL value
      rcc_val |= (mosc_source.unwrap() as u32) << 6;
    } else {
      // Disable MOSC
      rcc_val &= !(1 << 0);
    }

    rcc2_val |= 1 << 31; // Set USERCC2
    rcc2_val |= 1 << 30; // Set DIV400

    if use_pll {
      rcc_val  &= !(1 << 13);
      rcc2_val &= !(1 << 13);
    } else {
      rcc_val  |= 1 << 13;
      rcc2_val |= 1 << 13;
    }

    rcc.write32(rcc_val);
    rcc.write32(rcc2_val);

    match div {
      Some(div) => {
        // Configure system divisor
        rcc2_val &= !(0x7f << 22);
        rcc2_val |= (div as u32) << 22;
        rcc_val  |= 1 << 22;
      }
      _ => (),
    }

    let pllstat = Reg::new(super::BASE + PLLSTAT_OFFSET);

    if use_pll {
      let misc = Reg::new(super::BASE + MISC_OFFSET);

      // Clear any pending PLL lock interrupt
      misc.bitband_write(6, true);

      // Wait till PLL is locked
      loop {
        if pllstat.read32() & 1 != 0 {
          // PLL locked
          break;
        }
      }

      // Remove PLL bypass
      rcc_val  &= !(1 << 11);
      rcc2_val &= !(1 << 11);
    }

    rcc.write32(rcc_val);
    rcc2.write32(rcc2_val);
  }

  //// Raw Interrupt Status
  const RIS_OFFSET: u32 = 0x50;

  //// Masked Interrupt Status and Clear
  const MISC_OFFSET: u32 = 0x58;

  /// Run-mode clock configuration
  const RCC_OFFSET: u32 = 0x60;

  /// Run-mode clock configuration 2
  const RCC2_OFFSET: u32 = 0x70;

  /// PLL Status
  const PLLSTAT_OFFSET: u32 = 0x168;
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
