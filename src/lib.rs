use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::num::FpCategory::{Infinite, Nan};

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct ff64 {
    x: f64,
}

impl ff64 {
    pub fn from(x: f64) -> ff64 {
        match x.classify() {
            Nan | Infinite => panic!("Cannot create `fix f64` from {}", x),
            _ => {}
        }

        ff64 { x }
    }
}

impl Eq for ff64 {}

impl Ord for ff64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Debug for ff64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fix {}", self.x)
    }
}

impl Display for ff64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fix {}", self.x)
    }
}

impl Hash for ff64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.x == 0.0 {
            (0.0 as f64).to_bits().hash(state);
        } else {
            self.x.to_bits().hash(state)
        }
    }
}

impl From<f64> for ff64 {
    fn from(x: f64) -> Self {
        ff64::from(x)
    }
}

impl From<ff64> for f64 {
    fn from(x: ff64) -> Self {
        x.x
    }
}

#[macro_export]
macro_rules! ff64 {
    ($x:literal) => {
        $crate::ff64::from($x)
    };
    ($x:expr) => {
        $crate::ff64::from($x)
    };
}
