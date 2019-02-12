//! Implement delay abstraction.

use crate::peripheral::SYST;
use crate::clock::Clock;
use core::time::Duration;
use core::cmp::min;

/// System timer (SysTick) as a delay provider
pub struct Delay<TC>
where
    TC : Clock
{
    syst: SYST,
    clocks: TC,
}

impl<TC> Delay<TC>
where
    TC : Clock
{
    /// Delay execution
    pub fn delay(&mut self, d: Duration) {
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
}
