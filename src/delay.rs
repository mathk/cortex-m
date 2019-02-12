//! Implement delay abstraction.

use crate::peripheral::SYST;
use crate::clock::Clock;
use core::time::Duration;

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
    fn delay(&mut self, d: Duration) {
        let ticks = self.clocks.get_syst_clock(&mut self.syst).ticks_in(d);
    }
}
