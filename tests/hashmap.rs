use fix_float::*;
use std::collections::HashMap;

#[test]
fn simple() {
    let mut h: HashMap<ff64, u32> = HashMap::new();

    h.insert(ff64!(0.0), 42);
    h.insert(ff64!(2.2), 4);
    h.insert(ff64!(4.4), 8);
    h.insert(ff64!(6.6), 12);

    assert_eq!(*h.get(&ff64!(0.0)).unwrap(), 42);
    assert_eq!(*h.get(&ff64!(2.2)).unwrap(), 4);
    assert_eq!(*h.get(&ff64!(4.4)).unwrap(), 8);
    assert_eq!(*h.get(&ff64!(6.6)).unwrap(), 12);
}

#[test]
fn zero() {
    let mut h: HashMap<ff64, u32> = HashMap::new();

    h.insert(ff64!(0.0), 42);
    h.insert(ff64!(-0.0), 43);

    assert_eq!(*h.get(&ff64!(0.0)).unwrap(), 43);
    assert_eq!(*h.get(&ff64!(-0.0)).unwrap(), 43);
}
