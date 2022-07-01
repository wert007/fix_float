use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::num::FpCategory::{Infinite, Nan};
use std::ops::Deref;

macro_rules! _impl_ty {
    ($base:ty, $new:ident) => {
        #[allow(non_camel_case_types)]
        #[derive(Default, PartialEq, PartialOrd, Clone, Copy)]
        pub struct $new {
            x: $base,
        }

        impl $new {
            pub fn from(x: $base) -> $new {
                match x.classify() {
                    Nan | Infinite => {
                        panic!("Cannot create `fix {}` from {}", stringify!($base), x)
                    }
                    _ => {}
                }

                $new { x }
            }
        }

        impl Eq for $new {}

        impl Ord for $new {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        impl Debug for $new {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "fix {}", self.x)
            }
        }

        impl Display for $new {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "fix {}", self.x)
            }
        }

        impl Hash for $new {
            fn hash<H: Hasher>(&self, state: &mut H) {
                if self.x == 0.0 {
                    (0.0 as $base).to_bits().hash(state);
                } else {
                    self.x.to_bits().hash(state)
                }
            }
        }

        impl From<$base> for $new {
            fn from(x: $base) -> Self {
                $new::from(x)
            }
        }

        impl From<$new> for $base {
            fn from(x: $new) -> Self {
                x.x
            }
        }

        impl Deref for $new {
            type Target = $base;

            fn deref(&self) -> &Self::Target {
                &self.x
            }
        }

        #[macro_export]
        macro_rules! $new {
            ($x:literal) => {
                $crate::$new::from($x)
            };
            ($x:expr) => {
                $crate::$new::from($x)
            };
        }
    };
}

_impl_ty!(f64, ff64);
_impl_ty!(f32, ff32);
