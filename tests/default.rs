use fix_float::*;

#[test]
fn f64() {
    let a = ff64::default();

    assert_eq!(f64::from(a), f64::default());
}

#[test]
fn f32() {
    let a = ff32::default();

    assert_eq!(f32::from(a), f32::default());
}
