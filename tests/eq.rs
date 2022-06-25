use fix_float::*;
use rand;

#[test]
fn zero() {
    let a = ff64!(0.0);
    let b = ff64!(0.0);
    let c = ff64!(-0.0);

    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn random() {
    for _ in 0..1000 {
        let random_float = rand::random();
        let a = ff64!(random_float);
        let b = ff64!(random_float);

        assert_eq!(a, b);
    }
}
