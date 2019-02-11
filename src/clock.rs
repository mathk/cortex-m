//! Clock trait for Cortex-M.
//!
//! Access the SysTick peripheral and provide timing abstraction

#![allow(missing_docs)]
use core::ops::Div;
use core::time::Duration;
use crate::peripheral::syst::SystClkSource;
use crate::peripheral::SYST;


pub enum FreqRange {
    MegaHertz = 1_000_000,
    KiloHertz = 1_000,
    Hertz = 1,
}

impl FreqRange {
    fn scale_down(&self) -> Option<FreqRange> {
        match self {
            FreqRange::MegaHertz => Some(FreqRange::KiloHertz),
            FreqRange::KiloHertz => Some(FreqRange::Hertz),
            FreqRange::Hertz => None
        }
    }
}

/// Frequency abstraction
///
/// Using the frequency we can calculate the counter for some delay
pub struct Frequency {
    resolution: FreqRange,
    value: u32,
}

impl Frequency {
    pub fn tick(&self) -> Duration {
        match self.resolution {
            FreqRange::MegaHertz => Duration::from_nanos(1_000 / self.value as u64),
            FreqRange::KiloHertz => Duration::from_nanos(1_000_000 / self.value as u64),
            FreqRange::Hertz => Duration::from_nanos(1_000_000_000 / self.value as u64),
        }
    }
}

impl Div<u32> for Frequency {
    type Output = Option<Frequency>;

    fn div(self, rhs: u32) -> Option<Frequency> {
        let mut value = self.value;
        let mut res = Some(self.resolution);
        while res.is_some() && value % rhs < value {
            value = value * 1_000;
            res = res.and_then(|r| r.scale_down())
        }
        res.map(|r| Frequency {resolution: r, value: value / rhs })
    }
}

/// The main clock trait
pub trait Clock {

    fn get_external_syst_clock(&self) -> Frequency;
    fn get_core_clock(&self) -> Frequency;

    fn get_syst_clock(&self, syst: & mut SYST) -> Frequency {
        match syst.get_clock_source() {
            SystClkSource::External => self.get_external_syst_clock(),
            SystClkSource::Core => self.get_core_clock()
        }
    }
}
