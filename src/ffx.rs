use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::num::FpCategory::{Infinite, Nan};
use std::ops::Deref;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Error type when try_from is invoked
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorTryFrom {
    /// Happens when the value given is Nan (any type of Nan)
    CannotFixNan,
    /// Happens when the value given is Infinity (any type of Infinity)
    CannotFixInfinity,
}

macro_rules! _impl_ty {
    ($base:ty, $new:ident) => {
        doc_comment! {
            concat!(
        "Abstract wrapper for fix ", stringify!($base), ", or ", stringify!($new), " for short.

This wrapper implements:
 - Default
 - Copy
 - Clone
 - Debug
 - Display
 - PartialEq
 - Eq
 - PartialOrd
 - Ord
 - Hash
 - Add
 - Sub
 - Mul
 - Div
 - AddAssign
 - SubAssign
 - MulAssign
 - DivAssign
 - Deref<Target = ", stringify!($base), ">
 - TryFrom", "<", stringify!($base), ">

```
# use fix_float::*;
# fn main() {
	let x: ", stringify!($base), " =  42.42;
	let a: ", stringify!($new), " = ", stringify!($new), "::try_from(x).unwrap();
	let b: ", stringify!($new), " = x.try_into().unwrap();
	let c: ", stringify!($new), " = ", stringify!($new), "!(x);
# }
```
",
            ),

            #[allow(non_camel_case_types)]
            #[derive(Default, Clone, Copy)]
			#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            pub struct $new {
                x: $base,
            }

            impl $new {
                #[inline]
                fn is_fixable(x: $base) -> Option<ErrorTryFrom> {
                    match x.classify() {
                        Nan => Some(ErrorTryFrom::CannotFixNan),
                        Infinite => Some(ErrorTryFrom::CannotFixInfinity),
                        _ => None,
                    }
                }

                #[inline]
                fn assert(x: $base) -> $new {
                    Self::try_from(x).expect("Assertion failed")
                }

                #[inline]
                fn try_from(x: $base) -> Result<$new, ErrorTryFrom> {
                    match Self::is_fixable(x) {
                        Some(err) => Err(err),
                        None => Ok($new {
                            x: if (x == (0.0 as $base)) {
                                0.0 as $base
                            } else {
                                x
                            },
                        }),
                    }
                }

                #[inline]
                fn my_partial_cmp(lhs: &$new, rhs: &$new) -> Option<Ordering> {
                    Some(lhs.x.total_cmp(&rhs.x))
                }

                #[inline]
                fn my_cmp(lhs: &$new, rhs: &$new) -> Ordering {
                    lhs.x.total_cmp(&rhs.x)
                }

                #[inline]
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    f.pad(&format!("fix {}", self.x))
                }
            }

            impl PartialEq for $new {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    self.x == other.x
                }
            }

            impl std::ops::Add for $new {
                type Output = $new;
                #[inline]
                fn add(self, other: Self) -> $new {
                    $new::assert(self.x + other.x)
                }
            }

            impl std::ops::Sub for $new {
                type Output = $new;
                #[inline]
                fn sub(self, other: Self) -> $new {
                    $new::assert(self.x - other.x)
                }
            }

            impl std::ops::Mul for $new {
                type Output = $new;
                #[inline]
                fn mul(self, other: Self) -> $new {
                    $new::assert(self.x * other.x)
                }
            }

            impl std::ops::Div for $new {
                type Output = $new;
                #[inline]
                fn div(self, other: Self) -> $new {
                    $new::assert(self.x / other.x)
                }
            }
            impl std::ops::AddAssign for $new {
                #[inline]
                fn add_assign(&mut self, other: Self) {
                    *self = $new::assert(self.x + other.x);
                }
            }

            impl std::ops::SubAssign for $new {
                #[inline]
                fn sub_assign(&mut self, other: Self) {
                    *self = $new::assert(self.x - other.x);
                }
            }

            impl std::ops::MulAssign for $new {
                #[inline]
                fn mul_assign(&mut self, other: Self) {
                    *self = $new::assert(self.x * other.x);
                }
            }

            impl std::ops::DivAssign for $new {
                #[inline]
                fn div_assign(&mut self, other: Self) {
                    *self = $new::assert(self.x / other.x);
                }
            }

            impl Eq for $new {}

            impl PartialOrd for $new {
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Self::my_partial_cmp(self, other)
                }
            }

            impl Ord for $new {
                #[inline]
                fn cmp(&self, other: &Self) -> Ordering {
                    Self::my_cmp(self, other)
                }
            }

            impl Debug for $new {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    self.fmt(f)
                }
            }

            impl Display for $new {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    self.fmt(f)
                }
            }

            impl Hash for $new {
                fn hash<H: Hasher>(&self, state: &mut H) {
                    self.x.to_bits().hash(state)
                }
            }

            impl From<$new> for $base {
                #[inline]
                fn from(x: $new) -> Self {
                    x.x
                }
            }

            impl TryFrom<$base> for $new {
                type Error = ErrorTryFrom;

                #[inline]
                fn try_from(value: $base) -> Result<Self, Self::Error> {
                    Self::try_from(value)
                }
            }

            impl Deref for $new {
                type Target = $base;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.x
                }
            }

            doc_comment!{
                concat!("Macro that does try_from and unwraps it.

`", stringify!($new), "!(x)` <=> `fix_float::", stringify!($new), "::try_from(x).unwrap()`

You should use this macro when you are sure that you are not having Nan or Infinity. Otherwise, it will panic.

				"),
                #[macro_export]
                macro_rules! $new {
                    ($x:literal) => {
                        $crate::$new::try_from($x).unwrap()
                    };
                    ($x:expr) => {
                        $crate::$new::try_from($x).unwrap()
                    };
                }
            }
        }
    };
}

_impl_ty!(f64, ff64);
_impl_ty!(f32, ff32);

#[cfg(test)]
mod tests {
    use super::*;

    const F64_COMMON_FLOATS: &[f64] = &[
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

    const F32_COMMON_FLOATS: &[f32] = &[
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
    fn ff64_common_floats() {
        for &x in F64_COMMON_FLOATS {
            let fx = ff64!(x);
            assert_eq!(x, *fx)
        }
    }

    #[test]
    #[should_panic]
    fn ff64_nan() {
        let _ = ff64!(f64::NAN);
    }

    #[test]
    #[should_panic]
    fn ff64_inf() {
        let _ = ff64!(f64::INFINITY);
    }

    #[test]
    #[should_panic]
    fn ff64_neg_inf() {
        let _ = ff64!(f64::NEG_INFINITY);
    }

    #[test]
    fn ff32_common_floats() {
        for &x in F32_COMMON_FLOATS {
            let fx = ff32!(x);
            assert_eq!(x, *fx)
        }
    }

    #[test]
    #[should_panic]
    fn ff32_nan() {
        let _ = ff32!(f32::NAN);
    }

    #[test]
    #[should_panic]
    fn ff32_inf() {
        let _ = ff32!(f32::INFINITY);
    }

    #[test]
    #[should_panic]
    fn ff32_neg_inf() {
        let _ = ff32!(f32::NEG_INFINITY);
    }

    #[test]
    fn default_ff64() {
        let a = ff64::default();

        assert_eq!(f64::from(a), f64::default());
    }

    #[test]
    fn default_ff32() {
        let a = ff32::default();

        assert_eq!(f32::from(a), f32::default());
    }

    #[test]
    fn mem_f64() {
        assert_eq!(std::mem::size_of::<f64>(), std::mem::size_of::<ff64>())
    }

    #[test]
    fn mem_f32() {
        assert_eq!(std::mem::size_of::<f32>(), std::mem::size_of::<ff32>())
    }

    #[test]
    fn eq_zero() {
        let a = ff64!(0.0);
        let b = ff64!(0.0);
        let c = ff64!(-0.0);

        assert_eq!(a, b);
        assert_eq!(a, c);
    }

    #[test]
    fn eq_fuzzy() {
        for _ in 0..10000 {
            let random_float = rand::random();
            let a = ff64!(random_float);
            let b = ff64!(random_float);

            assert_eq!(a, b);
        }
    }

    #[test]
    fn ord_fuzzy() {
        for _ in 0..10000 {
            let a = rand::random();
            let b = rand::random();
            let fa = ff64!(a);
            let fb = ff64!(b);

            assert_eq!(a.partial_cmp(&b), fa.partial_cmp(&fb));
            assert_eq!(a.partial_cmp(&b).unwrap(), fa.cmp(&fb));
        }
    }

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", ff64!(2.42)), "fix 2.42");
        assert_eq!(format!("{:?}", ff64!(2.42)), "fix 2.42");
        assert_eq!(format!("{:}", ff64!(2.42)), "fix 2.42");
        assert_eq!(format!("{:10}", ff64!(2.42)), "fix 2.42  ");
        assert_eq!(format!("{:15}", ff64!(2.42)), "fix 2.42       ");
        assert_eq!(format!("{:0<15}", ff64!(2.42)), "fix 2.420000000");
        assert_eq!(format!("{:>15}", ff64!(2.42)), "       fix 2.42");
        assert_eq!(format!("{:0>15}", ff64!(2.42)), "0000000fix 2.42");
    }

    #[test]
    fn is_send() {
        fn assert_send<T: Send>() {}

        assert_send::<ff64>();
        assert_send::<ff32>();
    }

    #[test]
    fn is_sync() {
        fn assert_sync<T: Sync>() {}

        assert_sync::<ff64>();
        assert_sync::<ff32>();
    }

    #[test]
    fn try_from() {
        let a: ff64 = ff64::try_from(42.42).unwrap();
        let b: ff64 = (42.42).try_into().unwrap();
        let c: ff64 = ff64!(42.42);

        assert_eq!(a, b);
        assert_eq!(a, c);

        assert_eq!(*a, *b);
        assert_eq!(*a, *c);
    }

    #[test]
    fn try_from_nan() {
        let a = ff64::try_from(f64::NAN);
        assert!(a.is_err());
        assert_eq!(a.unwrap_err(), ErrorTryFrom::CannotFixNan);
    }

    #[test]
    fn try_from_inf() {
        let a = ff64::try_from(f64::INFINITY);
        assert!(a.is_err());
        assert_eq!(a.unwrap_err(), ErrorTryFrom::CannotFixInfinity);

        let b = ff64::try_from(f64::NEG_INFINITY);
        assert!(b.is_err());
        assert_eq!(b.unwrap_err(), ErrorTryFrom::CannotFixInfinity);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_json() {
        let a = ff64!(42.42);

        let json_a = serde_json::to_string(&a).unwrap();
        let a_json_a: ff64 = serde_json::from_str(&json_a).unwrap();

        assert_eq!(json_a, "{\"x\":42.42}");
        assert_eq!(a, a_json_a);
    }
}
