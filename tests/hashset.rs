use fix_float::*;
use std::collections::HashSet;

#[test]
fn simple() {
    let mut s: HashSet<ff64> = HashSet::new();

    s.insert(ff64!(1.0));
    s.insert(ff64!(2.0));
    s.insert(ff64!(3.0));

    assert_eq!(s.len(), 3);

    s.insert(ff64!(4.0));
    s.insert(ff64!(5.0));

    assert_eq!(s.len(), 5);
}

#[test]
fn zero() {
    let mut s: HashSet<ff64> = HashSet::new();

    s.insert(ff64!(0.0));
    assert_eq!(s.len(), 1);
    s.insert(ff64!(-0.0));
    assert_eq!(s.len(), 1);
}
