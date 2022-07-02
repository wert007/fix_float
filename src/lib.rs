//! Default float types in rust doesn't implement the standard traits Ord and Hash, which is theoretically correct but very annoying. This crate tries to handle some restriction on float values in order to finally implement these traits.
//!
//! Because we are talking about floating numbers, comparing and hashing are subtle. The first attempt is to create a wrapper around floating numbers such as ff64 and ff32, that prohibites edge values such as Nan (Not A Number) and +/- Infinite. This restriction is quite good because we would like to handle these edge cases seperataly and then concentrate on the 'happy path'.
//!
//! # Demo
//!
//! ```
//! use fix_float::*;
//! use rand;
//!
//! fn main() {
//! 	let mut v: Vec<ff64> = vec![];
//!
//! 	for _ in 0..10 {
//! 		v.push(ff64!(rand::random::<f64>()));
//! 	}
//!
//! 	println!("values:");
//! 	for &elem in &v {
//! 		println!("  {:.2}", *elem);
//! 	}
//!
//! 	v.sort();
//!
//! 	println!("values sorted:");
//! 	for &elem in &v {
//! 		println!("  {:.2}", *elem);
//! 	}
//! }
//! ```
//!
//! # Examples
//!
//! This should work fine!
//! ```
//! # use fix_float::*;
//! # fn main() {
//! let a = ff64!(42.42);
//! let b = ff64!(0f64);
//! # }
//! ```
//!
//! The three following snippets should panic!
//! ```should_panic
//! # use fix_float::*;
//! # fn main() {
//! let a = ff64!(f64::NAN);
//! # }
//! ```
//!
//! ```should_panic
//! # use fix_float::*;
//! # fn main() {
//! let a = ff64!(f64::INFINITY);
//! # }
//! ```
//!
//! ```should_panic
//! #use fix_float::*;
//! # fn main() {
//! let a = ff64!(f64::NEG_INFINITY);
//! # }
//! ```
//!
//! # Useful Datastructures
//!
//! A direct consequence is that we can now use fix floats in useful datastructures such as HashMap, HashSet, BinaryHeap. This is very useful when it comes to graph algorithms for instance.
//!
//! ```
//! # use std::collections::{HashMap, HashSet, BinaryHeap};
//! # use fix_float::*;
//! # fn main() {
//! let map: HashMap<ff64, ()> = HashMap::new();
//! let set: HashSet<ff64> = HashSet::new();
//! let heap: BinaryHeap<ff64> = BinaryHeap::new();
//! # }
//! ```
//!
//!
pub mod ffx;

pub use ffx::*;
