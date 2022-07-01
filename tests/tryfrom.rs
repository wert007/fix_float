use fix_float::ff64;
use fix_float::ErrorTryFrom;

#[test]
fn std() {
    let a: ff64 = ff64::try_from(42.42).unwrap();
    let b: ff64 = (42.42).try_into().unwrap();
    let c: ff64 = ff64!(42.42);

    assert_eq!(a, b);
    assert_eq!(a, c);

    assert_eq!(*a, *b);
    assert_eq!(*a, *c);
}

#[test]
fn nan() {
    let a = ff64::try_from(f64::NAN);
    assert!(a.is_err());
    assert_eq!(a.unwrap_err(), ErrorTryFrom::CannotFixNan);
}

#[test]
fn inf() {
    let a = ff64::try_from(f64::INFINITY);
    assert!(a.is_err());
    assert_eq!(a.unwrap_err(), ErrorTryFrom::CannotFixInfinite);

    let b = ff64::try_from(f64::NEG_INFINITY);
    assert!(b.is_err());
    assert_eq!(b.unwrap_err(), ErrorTryFrom::CannotFixInfinite);
}
