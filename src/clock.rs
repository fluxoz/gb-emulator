use std::time::Duration;
use std::thread::sleep;

// GB clock freq 4.194304 MHz
// some gb peripherals use an INVERTED clock
// rising edge of inverted clock may happen at the same time as the falling edge of the standard
// clock

static CLOCK_SPEED: u32 = 4_194_304;

pub struct Clock {
    speed: u32,
    ticks: u128,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            speed: CLOCK_SPEED, 
            ticks: 0,
        }
    }
    
    pub fn cycle(&mut self) {
        // actual period is 238.41857910156, maybe 238 is precise enough?
        let dur = Duration::from_nanos(238); 
        sleep(dur);
        self.ticks.wrapping_add(1);
    }
}
