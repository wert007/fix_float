use fix_float::*;

const COMMON_FLOATS: &'static [f64] = &[
    0.0,
    1.0,
    -1.0,
    42.42,
    f64::EPSILON,
    f64::MIN,
    f64::MAX,
    std::f64::consts::E,
    std::f64::consts::FRAC_1_PI,
    std::f64::consts::FRAC_1_SQRT_2,
    std::f64::consts::FRAC_2_PI,
    std::f64::consts::FRAC_2_SQRT_PI,
    std::f64::consts::FRAC_PI_2,
    std::f64::consts::FRAC_PI_3,
    std::f64::consts::FRAC_PI_4,
    std::f64::consts::FRAC_PI_6,
    std::f64::consts::FRAC_PI_8,
    std::f64::consts::LN_2,
    std::f64::consts::LN_10,
    std::f64::consts::LOG2_10,
    std::f64::consts::LOG2_E,
    std::f64::consts::LOG10_2,
    std::f64::consts::LOG10_E,
    std::f64::consts::PI,
    std::f64::consts::SQRT_2,
    std::f64::consts::TAU,
];

#[test]
fn common_floats() {
    for &x in COMMON_FLOATS {
        let _ = ff64::from(x);
    }
}

#[test]
#[should_panic]
fn nan() {
    let _ = ff64::from(f64::NAN);
}

#[test]
#[should_panic]
fn inf() {
    let _ = ff64::from(f64::INFINITY);
}

#[test]
#[should_panic]
fn neg_inf() {
    let _ = ff64::from(f64::NEG_INFINITY);
}
