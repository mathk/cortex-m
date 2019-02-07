
use core::ops::Div;
use core::time::Duration;


enum FreqRange {
    MegaHertz = 1_000_000,
    KiloHertz = 1_000,
    Hertz = 1,
}

impl FreqRange::MegaHertz {
    fn scale_down(&self) -> Option<FreqRange> {
        Some(FreqRange::KiloHertz)
    }
}

impl FreqRange::KiloHertz {
    fn scale_down(&self) -> Option<FreqRange> {
        Some(FreqRange::Hertz)
    }
}

impl FreqRange::Hertz {
    fn scale_down(&self) -> Option<FreqRange> {
        None
    }
}

pub struct Frequency {
    resolution: FreqRange,
    value: u32,
}

impl Frequency {
    fn tick(&self) -> Duration {
        match self.resolution {
            FreqRange::MegaHertz => Duration::from_nano(1_000 / self.value as u64),
            FreqRange::KiloHertz => Duration::from_nano(1_000_000 / self.value as u64),
            FreqRange::Hertz => Duration::from_nano(1_000_000_000 / self.value as u64),
        }
    }
}

impl Div<u32> for Frequency {
    type Output = Option<Frequency>;

    fn div(self, rhs: u32) -> Option<Frequency> {
        let mut value = self.value;
        let mut res = Some(self.resolution);
        while (let Some(res) = res) && value % rhs < value {
            value = value * 1_000;
            res = res.scale_down()
        }
        res.map(|r| Frequency {resolution: r, value: value / rhs })
    }
}

pub trait Clock {
    fn core_clock() -> Frequency;
}
