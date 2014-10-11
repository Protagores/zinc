#![feature(phase)]
#![crate_type="staticlib"]
#![no_std]

extern crate core;
extern crate zinc;
#[phase(plugin)] extern crate macro_platformtree;

platformtree!(
  tm4c123gh6pm@mcu {
    clock {
      source = "MOSC";
      /* Y2 16Mhz oscillator on launchpad board */
      source_frequency = 16_000_000;
    }

    gpio {
        PortF {
            led1@1 { direction = "out"; }
            led2@2 { direction = "out"; }
      }
    }
  }

  os {
    single_task {
      loop = "run";
      args {
        led1 = &led1;
        led2 = &led2;
      }
    }
  }
)

#[no_split_stack]
pub fn run(args: &pt::run_args) {
    use zinc::hal::pin::GPIO;

    args.led1.set_high();
    args.led2.set_high();
}
