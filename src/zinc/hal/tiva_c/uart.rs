// Zinc, the bare metal stack for rust.
// Copyright 2014 Lionel Flandrin <lionel@svkt.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! UART configuration

use hal::tiva_c::sysctl;
use hal::tiva_c::io::Reg;

use drivers::chario::CharIO;
use hal::uart;

#[path="../../util/ioreg.rs"] mod ioreg;
#[path="../../util/wait_for.rs"] mod wait_for;

/// There are 8 UART instances in total
#[allow(missing_doc)]
pub enum UARTID {
  UART0,
  UART1,
  UART2,
  UART3,
  UART4,
  UART5,
  UART6,
  UART7,
}

/// Structure describing a single UART
pub struct UART {
  /// Base address for this UART
  base    : u32,
}

impl UART {
  /// Create and setup a UART.
  pub fn new(id:        UARTID,
             baudrate:  uint,
             word_len:  u8,
             parity:    uart::Parity,
             stop_bits: u8) -> UART {

    let (periph, base) = match id {
      UART0 => (sysctl::periph::uart::UART_0,  UART_0_BASE),
      UART1 => (sysctl::periph::uart::UART_1,  UART_1_BASE),
      UART2 => (sysctl::periph::uart::UART_2,  UART_2_BASE),
      UART3 => (sysctl::periph::uart::UART_3,  UART_3_BASE),
      UART4 => (sysctl::periph::uart::UART_4,  UART_4_BASE),
      UART5 => (sysctl::periph::uart::UART_5,  UART_5_BASE),
      UART6 => (sysctl::periph::uart::UART_6,  UART_6_BASE),
      UART7 => (sysctl::periph::uart::UART_7,  UART_7_BASE),
    };

    let uart = UART { base: base };

    periph.ensure_enabled();

    uart.configure(baudrate, word_len, parity, stop_bits);

    uart
  }

  /// Configure the UART
  fn configure(&self,
               baudrate:  uint,
               word_len:  u8,
               parity:    uart::Parity,
               stop_bits: u8) {
    let sysclk = 16_000_000u;

    /* compute the baud rate divisor rounded to the nearest */
    let brd = ((((sysclk / 16) << 6) + baudrate / 2) / baudrate) as u32;

    let ctl = Reg::new(self.base + CTL);
    /* Disable the UART before configuration */
    ctl.bitband_write(0, false);

    /* Enable TX */
    ctl.bitband_write(8, true);
    /* Disable RX */
    ctl.bitband_write(9, false);
    ctl.bitband_write(5, false);

    ctl.write32(0x301);

    let ibrd = Reg::new(self.base + IBRD);
    ibrd.write32(brd >> 6);

    let fbrd = Reg::new(self.base + FBRD);
    fbrd.write32(brd & ((1 << 6) - 1));

    let lcrh = Reg::new(self.base + CRH);

    /* This is where we can do the real config. */
    lcrh.write32(0x70);

    /* Enable the UART */
    ctl.write32(0x301);
  }
}

impl CharIO for UART {
  fn putc(&self, value: char) {
    let data = Reg::new(self.base + DATA);
    let fr   = Reg::new(self.base + FR);

    while fr.read32() & (1 << 5) != 0 {
    }

    data.write32(value as u32);
  }
}

static UART_0_BASE: u32 = 0x4000C000;
static UART_1_BASE: u32 = 0x4000D000;
static UART_2_BASE: u32 = 0x4000E000;
static UART_3_BASE: u32 = 0x4000F000;
static UART_4_BASE: u32 = 0x40010000;
static UART_5_BASE: u32 = 0x40011000;
static UART_6_BASE: u32 = 0x40012000;
static UART_7_BASE: u32 = 0x40013000;

static DATA     : u32 = 0x00;
static FR       : u32 = 0x18;
static CTL      : u32 = 0x30;
static IBRD     : u32 = 0x24;
static FBRD     : u32 = 0x28;
static CRH      : u32 = 0x2c;
