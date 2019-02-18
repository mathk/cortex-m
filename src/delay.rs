//! Implement delay abstraction.

#![allow(missing_docs)]
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

/// Capture an instant from a delay.
pub struct SysTickInstant<T>
where T : Clocks
{
    delay: SysTickDelay<T>,
}

impl<T> SysTickInstant<T>
where T : Clocks
{
    fn now(delay: SysTickDelay<T>) -> Self {
        SysTickInstant {
            delay,
        }
    }

    pub fn elapse(&mut self) -> Duration {
        if self.delay.has_wrapped() {
            panic!("Can not tell the elapse time as we have wrapped.")
        }
        self.delay.tick() * (0x0FF_FFFF - self.delay.get_current())
    }

    pub fn stop(self) -> SysTickDelay<T> {
        self.delay.stop()
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

    pub fn start(mut self) ->  SysTickInstant<T> {
        self.syst.set_reload(0x00FF_FFFF);
        self.syst.clear_current();
        self.syst.enable_counter();
        SysTickInstant::now(self)
    }

    pub fn stop(mut self) -> Self {
        self.syst.disable_counter();
        self
    }

    pub fn tick(&mut self) -> Duration {
        self.clocks.get_syst_clock(&mut self.syst).tick()
    }

    pub fn has_wrapped(&mut self) -> bool {
        self.syst.has_wrapped()
    }

    pub fn get_current(&mut self) -> u32 {
        SYST::get_current()
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
            ticks -= current;
            while !self.has_wrapped() {}
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
            self.syst.enable_interrupt();
            self.syst.enable_counter();
            ticks -= current;
            while !self.has_wrapped() {
                wfe()
            }
            self.syst.disable_counter();
            self.syst.disable_interrupt();
        }
    }

}
