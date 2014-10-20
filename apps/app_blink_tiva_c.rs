#![feature(phase)]
#![crate_type="staticlib"]
#![no_std]

extern crate core;
extern crate zinc;
#[phase(plugin)] extern crate macro_platformtree;

platformtree!(
  tiva_c@mcu {
    clock {
      source = "MOSC";
      /* Y2 16Mhz oscillator on launchpad board */
      source_frequency = 16_000_000;
    }

    gpio {
        PortA {
          led1@1 { direction = "out"; }
          }
        PortF {
        
            led2@2 { direction = "out"; }
      }
    }

    timer {
      /* The mcu contain both 16/32bit and "wide" 32/64bit timers. */
      timer@w0 {
        /* prescale sysclk to 1Mhz since the wait code expects 1us
         * granularity */
        prescale = 80;
        mode = "periodic";
      }
    }
  }

  os {
    single_task {
      loop = "run";
      args {
        timer = &timer;
        led1 = &led1;
        led2 = &led2;
      }
    }
  }
)

pub fn run(args: &pt::run_args) {
  use zinc::hal::pin::GPIO;
  use zinc::hal::timer::Timer;

  use core::option::{Some};

  use zinc::hal::tiva_c::sysctl::clock::{MOSC, X16_0MHz, sysclk_configure};

  sysclk_configure(MOSC, Some(X16_0MHz), true, Some(5));

  loop {
    args.led1.set_high();
    args.led2.set_low();

    args.timer.wait(1);

    args.led1.set_low();
    args.led2.set_high();

    args.timer.wait(1);
  }
}
