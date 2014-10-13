//! Pin configuration
//! Allows GPIO configuration
//! Pin muxing not implemented yet.

use hal::pin::{GPIO, GPIODirection, In, Out, GPIOLevel, High, Low};
use hal::tiva_c::sysctl;
use hal::tiva_c::io::Reg;

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
  /// Base address for the port containing the pin
  base:  u32,
  /// Pin index in the port
  index: u8,
}

impl Pin {
  /// Create and configure a Pin
  pub fn new(pid:       PortID,
             pin_index: u8,
             dir:       GPIODirection) -> Pin {

    // Retrieve GPIO port peripheral to enable it
    let (periph, base) = match pid {
      PortA => (sysctl::periph::gpio::PORT_A, PORT_A_BASE),
      PortB => (sysctl::periph::gpio::PORT_B, PORT_B_BASE),
      PortC => (sysctl::periph::gpio::PORT_C, PORT_C_BASE),
      PortD => (sysctl::periph::gpio::PORT_D, PORT_D_BASE),
      PortE => (sysctl::periph::gpio::PORT_E, PORT_E_BASE),
      PortF => (sysctl::periph::gpio::PORT_F, PORT_F_BASE),
    };

    periph.ensure_enabled();

    let pin = Pin { base: base, index: pin_index };

    pin.configure(dir);

    pin
  }

  /// Configure GPIO pin
  fn configure(&self, dir: GPIODirection) {
    // Disable the GPIO during reconfig
    let den = Reg::new(self.base + DEN);
    den.bitband_write(self.index, false);

    self.set_direction(dir);

    // Configure the "alternate function". 0 means GPIO, 1 means the port is
    // driven by another peripheral.
    let afsel = Reg::new(self.base + AFSEL);
    afsel.bitband_write(self.index, false);

    // We can chose to drive each GPIO at either 2, 4 or 8mA. Default to 2mA for
    // now.
    let drive = Reg::new(self.base + DR2R);
    drive.bitband_write(self.index, true);

    // XXX TODO: configure open drain/pull up/pull down/slew rate if necessary

    // Enable GPIO
    den.bitband_write(self.index, true);
  }

  /// Returns a register containing the address to read and write the level of a
  /// specific GPIO pin.
  ///
  /// Bits [9:2] of the address are a mask to address only specific pins in a
  /// port.
  fn data_reg(&self) -> Reg {
    let off: u32 = 1u32 << ((self.index as uint) + 2);

    Reg::new(self.base + DATA + off)
  }
}

impl GPIO for Pin {
  /// Sets output GPIO value to high.
  fn set_high(&self) {
    let r = self.data_reg();

    r.write32(0xff);
  }

  /// Sets output GPIO value to low.
  fn set_low(&self) {
    let r = self.data_reg();

    r.write32(0x00);
  }

  /// Returns input GPIO level.
  fn level(&self) -> GPIOLevel {
    let r = self.data_reg();

    if r.read32() == 0 {
      Low
    } else {
      High
    }
  }

  /// Sets output GPIO direction.
  fn set_direction(&self, dir: GPIODirection) {
    let reg = Reg::new(self.base + DIR);
    reg.bitband_write(self.index,
                      match dir {
                        In  => false,
                        Out => true,
                      });
  }
}

static PORT_A_BASE: u32 = 0x40004000;
static PORT_B_BASE: u32 = 0x40005000;
static PORT_C_BASE: u32 = 0x40006000;
static PORT_D_BASE: u32 = 0x40007000;
static PORT_E_BASE: u32 = 0x40024000;
static PORT_F_BASE: u32 = 0x40025000;

// Register offsets from port base
static DATA    : u32 = 0x000;
static DIR     : u32 = 0x400;
static AFSEL   : u32 = 0x420;
static DR2R    : u32 = 0x500;
static DEN     : u32 = 0x51C;
