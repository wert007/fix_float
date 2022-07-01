use fix_float::ff64;

#[test]
fn std() {
    let a = ff64::from(42.42);
    let b = ff64!(42.42);
    let c: ff64 = (42.42).into();

    assert_eq!(a, b);
    assert_eq!(a, c);

    assert_eq!(*a, *b);
    assert_eq!(*a, *c);
}
