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

    uart {
      uart@0 {
        mode = "9600,8n1";
      }
    }

    gpio {
      PortA {
        uart_rx@0 {
          direction = "in";
          function  = 1;
        }
        uart_tx@1 {
          direction = "out";
          function  = 1;
        }
      }
      PortF {
        led1@1 { direction = "out"; }
        led2@3 { direction = "out"; }
      }
    }
  }

  os {
    single_task {
      loop = "run";
      args {
        timer = &timer;
        uart = &uart;
        led1 = &led1;
        led2 = &led2;
      }
    }
  }
)

#[no_stack_check]
fn run(args: &pt::run_args) {
  use zinc::drivers::chario::CharIO;
  use zinc::hal::pin::GPIO;

  args.led1.set_high();

  args.uart.putc('@');

  args.led1.set_low();
  args.led2.set_high();
  
  loop {
      args.uart.putc('|');
  }


  //args.led2.set_high();

  // let mut i = 0;
  // loop {
  //   //args.txled.set_high();
  //   args.uart.puts("Waiting for ");
  //   //args.uart.puti(i);
  //   args.uart.puts(" seconds...\n");

  //   i += 1;
  //   args.txled.set_low();

  //   args.timer.wait(1);
  // }
}
