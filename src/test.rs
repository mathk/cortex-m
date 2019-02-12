use crate::clock::FreqRange;
use crate::clock::U32Ext;

#[test]
fn divide() {
    assert_eq!((1.mhz() / 2).into_kilo().resolution, FreqRange::KiloHertz);
    assert_eq!((1.mhz() / 2).into_kilo().numerator, 500);
    assert_eq!((1.mhz() / 8000).into_hertz().resolution, FreqRange::Hertz);
    assert_eq!((1.mhz() / 8000).into_hertz().numerator, 125);
    assert_eq!((1.mhz() / 80000).into_hertz().resolution, FreqRange::Hertz);
    assert_eq!((1.mhz() / 80000).into_hertz().numerator, 12);

}

#[test]
fn tick() {
    assert_eq!(1.mhz().tick(), 1.us());
    assert_eq!(1.khz().tick(), 1.ms());
    assert_eq!(1.hz().tick(), 1.s());
    assert_eq!(1.hz().ticks_in(2.s()), 2);
}

#[test]
fn ticks_in() {
    assert_eq!(1.khz().ticks_in(2.s()), 2_000);
    assert_eq!(1.mhz().ticks_in(2.s()), 2_000_000);
    assert_eq!(2.mhz().ticks_in(2.s()), 4_000_000);
    assert_eq!(2.mhz().ticks_in(2.us()), 4);
}