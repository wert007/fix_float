#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! Default float types in rust doesn't implement the standard traits Ord and Hash, which is theoretically correct but very annoying. This crate tries to handle some restriction on float values in order to finally implement these traits.
//!
//! Because we are talking about floating numbers, comparing and hashing are subtle. We create a wrapper around floating numbers named ff64 and ff32, that prohibites edge values such as Nan (Not A Number) and +/- Infinite. This restriction is quite good because we would like to handle these edge cases seperataly and then concentrate on the 'happy path'.
//!
//! A fix float is a floating number that can be fixed. In this paradigm, every normal, subnormal or zero number can be fixed, when Nan (Not A Number) and Infinity can't.
//!
//! Furthermore, not a single mathematical operation can be done, this is to prevent any forbidden operations that could lead to Nan or Infinity, such as `(-1f64).sqrt()` or `0f64.recip()`.
//!
//! A negative zero (-0), is transformed into a positive zero.
//!
//! # Getting started
//!
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
//! // NAN
//! let a = ff64!(0f64 / 0f64);
//! # }
//! ```
//!
//! ```should_panic
//! # use fix_float::*;
//! # fn main() {
//! // INFINITY
//! let a = ff64!(1f64 / 0f64);
//! # }
//! ```
//!
//! ```should_panic
//! # use fix_float::*;
//! # fn main() {
//! // NEG_INFINITY
//! let a = ff64!(-1f64 / 0f64);
//! # }
//! ```
//!
//! # Examples
//!
//! ## Separate edge-cases numbers and sort valid fix floating numbers.
//!
//! ```
//! use fix_float::*;
//!
//! // inputs: A random list of float numbers
//! // outputs: A tuple of three elements:
//! //    .0: sorted list of fix floats numbers
//! //    .1: numbers of NAN
//! //    .2: numbers of INFINITY (positive or negative)
//! fn double_triage(v: Vec<f64>) -> (Vec<ff64>, usize, usize) {
//! 	let mut vff64 = Vec::new();
//! 	let mut nb_nan = 0;
//! 	let mut nb_infinity = 0;
//!
//! 	for &elem in &v {
//! 		match ff64::try_from(elem) {
//! 			Ok(x) => vff64.push(x),
//! 			Err(ErrorTryFrom::CannotFixNan) => nb_nan += 1,
//! 			Err(ErrorTryFrom::CannotFixInfinity) => nb_infinity += 1,
//! 		}
//! 	}
//!
//! 	vff64.sort();
//!
//! 	(vff64, nb_nan, nb_infinity)
//! }
//!
//! # fn main() {
//! # 	let v = vec![f64::NAN, 42.42, f64::INFINITY, 21.21, 84.84, f64::NAN, f64::NAN, f64::NEG_INFINITY];
//! # 	let (sv, nb_nan, nb_infinity) = double_triage(v);
//! # 	assert_eq!(nb_nan, 3);
//! # 	assert_eq!(nb_infinity, 2);
//! # 	assert_eq!(sv.len(), 3);
//! # 	assert_eq!(*sv[0], 21.21);
//! # 	assert_eq!(*sv[1], 42.42);
//! # 	assert_eq!(*sv[2], 84.84);
//! # }
//! ```
//!
//! ## Sort fix floating numbers
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
//! # Useful Datastructures
//!
//! A direct consequence is that we can now use fix floats in useful datastructures such as HashMap, HashSet, BinaryHeap. For instance, this is very useful when it comes to graph algorithms.
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

#[macro_use]
extern crate doc_comment;

/// Module that defines ff64 and ff32
pub mod ffx;

pub use ffx::*;
