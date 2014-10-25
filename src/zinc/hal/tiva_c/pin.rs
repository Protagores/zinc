//! Pin configuration
//! Allows GPIO configuration
//! Pin muxing not implemented yet.

use hal::pin::{GPIO, GPIODirection, In, Out, GPIOLevel, High, Low};
use hal::tiva_c::sysctl;
use hal::tiva_c::io;

/// The pins are accessed through ports. Each port has 8 pins and are identified
/// by a letter (PortA, PortB, etc...).
#[allow(missing_doc)]
pub enum PortID {
  PortA,
  PortB,
  PortC,
  PortD,
  PortE,
  PortF,
}

/// Structure describing a single HW pin
pub struct Pin {
  /// Timer register interface
  regs: &'static reg::Port,
  /// Pin index in the port
  index: uint,
}

impl Pin {
  /// Create and configure a Pin
  pub fn new(pid:       PortID,
             pin_index: u8,
             dir:       GPIODirection,
             function:  u8) -> Pin {

    // Retrieve GPIO port peripheral to enable it
    let (periph, regs) = match pid {
      PortA => (sysctl::periph::gpio::PORT_A, reg::PORT_A),
      PortB => (sysctl::periph::gpio::PORT_B, reg::PORT_B),
      PortC => (sysctl::periph::gpio::PORT_C, reg::PORT_C),
      PortD => (sysctl::periph::gpio::PORT_D, reg::PORT_D),
      PortE => (sysctl::periph::gpio::PORT_E, reg::PORT_E),
      PortF => (sysctl::periph::gpio::PORT_F, reg::PORT_F),
    };

    periph.ensure_enabled();

    let pin = Pin { regs: io::get_reg_ref(regs), index: pin_index as uint };

    pin.configure(dir, function);

    pin
  }

  /// Configure GPIO pin
  fn configure(&self, dir: GPIODirection, function: u8) {
    // Disable the GPIO during reconfig
    self.regs.den.set_den(self.index, false);

    self.set_direction(dir);

    // Configure the "alternate function". AFSEL 0 means GPIO, 1 means the port
    // is driven by another peripheral. When AFSEL is 1 the actual function
    // config goes into the CTL register.
    match function {
      0 => {
        self.regs.afsel.set_afsel(self.index, reg::GPIO);
      },
      f => {
        self.regs.afsel.set_afsel(self.index, reg::PERIPHERAL);

        self.regs.pctl.set_pctl(self.index, f as u32);
      }
    }

    // We can chose to drive each GPIO at either 2, 4 or 8mA. Default to 2mA for
    // now.
    self.regs.dr2r.set_dr2r(self.index, true);
    self.regs.dr4r.set_dr4r(self.index, false);
    self.regs.dr8r.set_dr8r(self.index, false);

    // XXX TODO: configure open drain/pull up/pull down/slew rate if necessary

    self.regs.odr.set_odr(self.index, false);
    self.regs.pur.set_pur(self.index, false);
    self.regs.pdr.set_pdr(self.index, false);

    // Enable GPIO
    self.regs.den.set_den(self.index, true);
  }

  fn set_level(&self, level: bool) {
    self.regs.data.set_data(self.index, level);
  }
}

impl GPIO for Pin {
  /// Sets output GPIO value to high.
  fn set_high(&self) {
    self.set_level(true);
  }

  /// Sets output GPIO value to low.
  fn set_low(&self) {
    self.set_level(false);
  }

  /// Returns input GPIO level.
  fn level(&self) -> GPIOLevel {
    match self.regs.data.data(self.index) {
      true  => High,
      false => Low,
    }
  }

  /// Sets output GPIO direction.
  fn set_direction(&self, dir: GPIODirection) {
    self.regs.dir.set_dir(self.index,
                          match dir {
                            In  => reg::INPUT,
                            Out => reg::OUTPUT,
                          });
  }
}

pub mod reg {
  //! Pin registers definition
  use util::volatile_cell::VolatileCell;
  use core::ops::Drop;

  ioregs!(Port = {
    0x3FC => reg32 data {
      //! Pin value
      0..7   => data[8]
    }

    0x400 => reg32 dir {
      //! Pin direction
      0..7   => dir[8] {
        0 => INPUT,
        1 => OUTPUT,
      }
    }

    0x420 => reg32 afsel {
      //! Pin alternate function
      0..7   => afsel[8] {
        0 => GPIO,
        1 => PERIPHERAL,
      }
    }

    0x500 => reg32 dr2r {
      //! Select 2mA drive strength
      0..7   => dr2r[8]
    }

    0x504 => reg32 dr4r {
      //! Select 4mA drive strength
      0..7   => dr4r[8]
    }

    0x508 => reg32 dr8r {
      //! Select 8mA drive strength
      0..7   => dr8r[8]
    }

    0x50C => reg32 odr {
      //! Configure pin as open drain
      0..7   => odr[8]
    }

    0x510 => reg32 pur {
      //! Enable pin pull-up
      0..7   => pur[8]
    }

    0x514 => reg32 pdr {
      //! Enable pin pull-down
      0..7   => pdr[8]
    }

    0x518 => reg32 slr {
      //! Slew rate control enable (only available for 8mA drive strength)
      0..7   => slr[8]
    }

    0x51C => reg32 den {
      //! Enable pin
      0..7   => den[8]
    }

    0x52C => reg32 pctl {
      //! Pin function selection when afsel is set for the pin.
      0..31   => pctl[8]
    }
  })

  pub const PORT_A: *const Port = 0x40004000 as *const Port;
  pub const PORT_B: *const Port = 0x40005000 as *const Port;
  pub const PORT_C: *const Port = 0x40006000 as *const Port;
  pub const PORT_D: *const Port = 0x40007000 as *const Port;
  pub const PORT_E: *const Port = 0x40024000 as *const Port;
  pub const PORT_F: *const Port = 0x40025000 as *const Port;
}
