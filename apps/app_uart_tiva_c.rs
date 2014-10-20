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

    timer {
      /* The mcu contain both 16/32bit and "wide" 32/64bit timers. */
      timer@w0 {
        /* prescale sysclk to 1Mhz since the wait code expects 1us
         * granularity */
        prescale = 16;
        mode = "periodic";
      }
    }


    gpio {
      PortA {
        uart_rx@0 {
          direction = "in";
          function  = 1;
        }
        uart_tx@1 {
          direction = "in";
          function  = 1;
        }
      }
      PortF {
        txled@2 { direction = "out"; }
      }
    }

    uart {
      uart@0 {
        mode = "115200,8n1";
      }
    }

  }

  os {
    single_task {
      loop = "run";
      args {
        timer = &timer;
        uart = &uart;
        txled = &txled;
        uart_tx = &uart_tx;
      }
    }
  }
)

fn run(args: &pt::run_args) {
  use zinc::drivers::chario::CharIO;
  use zinc::hal::timer::Timer;
  use zinc::hal::pin::GPIO;

  args.uart.puts("Hello, world\n");

  let mut i = 0;
  loop {
    args.txled.set_high();
    args.uart.puts("Waiting for ");
    args.uart.puti(i);
    args.uart.puts(" seconds...\n");

    i += 1;
    args.txled.set_low();

    args.timer.wait(1);
  }
}
