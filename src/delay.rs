//! Implement delay abstraction.

use crate::peripheral::SYST;
use crate::peripheral::syst::SystClkSource;
use crate::clock::Clocks;
use crate::asm::wfe;
use core::time::Duration;
use core::cmp::min;

/// Delay trait
pub trait Delay {

    /// Pause the execution for Duration.
    fn delay(&mut self, d: Duration);

    /// Pause execution assuming interrupt is enabled
    /// and correctly handler.
    fn delay_with_interrupt(&mut self, d: Duration) {
        // By default is a not optimal delay.
        self.delay(d);
    }
}

/// Delay base on Systick.
pub struct SysTickDelay<T>
where T : Clocks
{
    syst: SYST,
    clocks: T,
}

/// Delay using the SysTick timer
impl<T> SysTickDelay<T>
where
    T: Clocks
{

    /// Build a new SysTick timer base on external source clock.
    /// External clock is vendor dependent
    pub fn new_external(mut syst: SYST, clocks: T) -> Self {
        syst.set_clock_source(SystClkSource::External);
        SysTickDelay {
            syst,
            clocks
        }
    }
}


impl<T> Delay for SysTickDelay<T>
where
    T : Clocks
{
    /// Delay execution
    fn delay(&mut self, d: Duration) {
        const MAX_RVR: u64 = 0x00FF_FFFF;
        let mut ticks = self.clocks.get_syst_clock(&mut self.syst).ticks_in(d);

        while ticks != 0 {
            let current = min(MAX_RVR, ticks);
            self.syst.set_reload(current as u32);
            self.syst.clear_current();
            self.syst.enable_counter();
            ticks += current;
            while !self.syst.has_wrapped() {}
            self.syst.disable_counter();
        }
    }

    fn delay_with_interrupt(&mut self, d: Duration) {
        const MAX_RVR: u64 = 0x00FF_FFFF;
        let mut ticks = self.clocks.get_syst_clock(&mut self.syst).ticks_in(d);

        while ticks != 0 {
            let current = min(MAX_RVR, ticks);
            self.syst.set_reload(current as u32);
            self.syst.clear_current();
            self.syst.enable_counter();
            ticks += current;
            while !self.syst.has_wrapped() {
                wfe()
            }
            self.syst.disable_counter();
        }
    }

}
