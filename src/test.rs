use crate::clock::FreqRange;
use crate::clock::U32Ext;

#[test]
fn divide() {
    assert_eq!((1.mhz() / 2).unwrap().resolution, FreqRange::KiloHertz);
    assert_eq!((1.mhz() / 2).unwrap().value, 500);
}
