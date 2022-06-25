use fix_float::*;

const F64_COMMON_FLOATS: &'static [f64] = &[
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

const F32_COMMON_FLOATS: &'static [f32] = &[
    0.0,
    1.0,
    -1.0,
    42.42,
    f32::EPSILON,
    f32::MIN,
    f32::MAX,
    std::f32::consts::E,
    std::f32::consts::FRAC_1_PI,
    std::f32::consts::FRAC_1_SQRT_2,
    std::f32::consts::FRAC_2_PI,
    std::f32::consts::FRAC_2_SQRT_PI,
    std::f32::consts::FRAC_PI_2,
    std::f32::consts::FRAC_PI_3,
    std::f32::consts::FRAC_PI_4,
    std::f32::consts::FRAC_PI_6,
    std::f32::consts::FRAC_PI_8,
    std::f32::consts::LN_2,
    std::f32::consts::LN_10,
    std::f32::consts::LOG2_10,
    std::f32::consts::LOG2_E,
    std::f32::consts::LOG10_2,
    std::f32::consts::LOG10_E,
    std::f32::consts::PI,
    std::f32::consts::SQRT_2,
    std::f32::consts::TAU,
];

#[test]
fn f64_common_floats() {
    for &x in F64_COMMON_FLOATS {
        let fx = ff64!(x);
        assert_eq!(x, fx.unfix())
    }
}

#[test]
#[should_panic]
fn f64_nan() {
    let _ = ff64!(f64::NAN);
}

#[test]
#[should_panic]
fn f64_inf() {
    let _ = ff64!(f64::INFINITY);
}

#[test]
#[should_panic]
fn f64_neg_inf() {
    let _ = ff64!(f64::NEG_INFINITY);
}

#[test]
fn f32_common_floats() {
    for &x in F32_COMMON_FLOATS {
        let fx = ff32!(x);
        assert_eq!(x, fx.unfix())
    }
}

#[test]
#[should_panic]
fn f32_nan() {
    let _ = ff32!(f32::NAN);
}

#[test]
#[should_panic]
fn f32_inf() {
    let _ = ff32!(f32::INFINITY);
}

#[test]
#[should_panic]
fn f32_neg_inf() {
    let _ = ff32!(f32::NEG_INFINITY);
}
