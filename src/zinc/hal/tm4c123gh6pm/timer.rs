//! Timer configuration
//! This code should support both standand and wide timers

use hal::tm4c123gh6pm::sysctl;
use hal::tm4c123gh6pm::io::Reg;
use hal::timer;

/// There are 6 standard 16/32bit timers and 6 "wide" 32/64bit timers
#[allow(missing_doc)]
pub enum TimerId {
  Timer0,
  Timer1,
  Timer2,
  Timer3,
  Timer4,
  Timer5,
  TimerW0,
  TimerW1,
  TimerW2,
  TimerW3,
  TimerW4,
  TimerW5,
}

/// Timer modes
pub enum Mode {
  /// Periodic timer loops and restarts once the timeout is reached.
  Periodic,
  /// One shot timer is disabled once the timeout is reached.
  OneShot,
  /// RTC timer is based on the 32.768KHz clock and ticks at 1Hz
  RTC,
  /// EdgeCount timer counts rising/falling/both edge events on an
  /// external pin.
  EdgeCount,
  /// EdgeTime timer measures the time it takes for a rising/falling/both edge
  /// event to occur.
  EdgeTime,
  /// PWM mode can be used to generate a configurable square wave (frequence and
  /// duty cycle)
  PWM,
}

/// Structure describing a single timer counter (both 16/32bit and 32/64bit)
pub struct Timer {
  /// Base address for this counter
  base    : u32,
  /// True if the counter is wide 32/64bit
  wide    : bool,
  /// Current timer mode
  mode    : Mode,
}

impl Timer {
  /// Create and configure a Timer
  pub fn new(tid:      TimerId,
             mode:     Mode,
             prescale: u16) -> Timer {
    let (periph, base, wide) = match tid {
      Timer0  => (sysctl::periph::timer::TIMER_0,   TIMER_0_BASE,   false),
      Timer1  => (sysctl::periph::timer::TIMER_1,   TIMER_1_BASE,   false),
      Timer2  => (sysctl::periph::timer::TIMER_2,   TIMER_2_BASE,   false),
      Timer3  => (sysctl::periph::timer::TIMER_3,   TIMER_3_BASE,   false),
      Timer4  => (sysctl::periph::timer::TIMER_4,   TIMER_4_BASE,   false),
      Timer5  => (sysctl::periph::timer::TIMER_5,   TIMER_5_BASE,   false),
      TimerW0 => (sysctl::periph::timer::TIMER_W_0, TIMER_W_0_BASE, true),
      TimerW1 => (sysctl::periph::timer::TIMER_W_1, TIMER_W_1_BASE, true),
      TimerW2 => (sysctl::periph::timer::TIMER_W_2, TIMER_W_2_BASE, true),
      TimerW3 => (sysctl::periph::timer::TIMER_W_3, TIMER_W_3_BASE, true),
      TimerW4 => (sysctl::periph::timer::TIMER_W_4, TIMER_W_4_BASE, true),
      TimerW5 => (sysctl::periph::timer::TIMER_W_5, TIMER_W_5_BASE, true),
    };

    periph.ensure_enabled();

    let timer = Timer { base: base, wide: wide, mode: mode};

    timer.configure(prescale);

    timer
  }

  /// Configure timer registers
  /// XXX Only Periodic and OneShot modes are implemented so far
  pub fn configure(&self, prescale: u16) {

    let ctl = Reg::new(self.base + CTL);

    // Make sure the timer is disabled before making changes.
    ctl.bitband_write(0, false);

    let cfg = Reg::new(self.base + CFG);

    // Configure the timer as half-width so that we can use the prescaler
    cfg.write32(0x4);

    let amr = Reg::new(self.base + AMR);

    // Configure TAMR
    let mut amr_val = match self.mode {
      OneShot  => 0x1,
      Periodic => 0x2,
      _        => { return; /* Not implemented! */ },
    };

    // We need to count down in order for the prescaler to work as a
    // prescaler. If we count up it becomes a timer extension (i.e. it becomes
    // the MSBs of the counter).
    amr_val |= 0 << 4;

    amr.write32(amr_val);

    // Set maximum timeout value to overflow as late as possible
    let tailr = Reg::new(self.base + TAILR);

    tailr.write32(0xffffffff);

    // Set prescale value
    let apr = Reg::new(self.base + APR);

    apr.write32(prescale as u32);

    // Timer is now configured, we can enable it
    ctl.bitband_write(0, true);
  }
}

impl timer::Timer for Timer {
  /// Retrieve the current timer value
  #[inline(always)]
  fn get_counter(&self) -> u32 {
    let tav = Reg::new(self.base + TAV);

    // We count down, however the trait code expects that the counter increases,
    // so we just complement the value to get an increasing counter.
    !tav.read32()
  }
}

static TIMER_0_BASE: u32 = 0x40030000;
static TIMER_1_BASE: u32 = 0x40031000;
static TIMER_2_BASE: u32 = 0x40032000;
static TIMER_3_BASE: u32 = 0x40033000;
static TIMER_4_BASE: u32 = 0x40034000;
static TIMER_5_BASE: u32 = 0x40035000;

static TIMER_W_0_BASE: u32 = 0x40036000;
static TIMER_W_1_BASE: u32 = 0x40037000;
static TIMER_W_2_BASE: u32 = 0x4003C000;
static TIMER_W_3_BASE: u32 = 0x4003D000;
static TIMER_W_4_BASE: u32 = 0x4003E000;
static TIMER_W_5_BASE: u32 = 0x4003F000;

// Register offsets from timer base
static CFG     : u32 = 0x000;
static AMR     : u32 = 0x004;
static CTL     : u32 = 0x00C;
static TAILR   : u32 = 0x028;
static APR     : u32 = 0x038;
static TAV     : u32 = 0x050;
