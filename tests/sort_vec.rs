use std::iter::zip;

use fix_float::*;

fn sort_vec_wrapper(s: usize) {
    let mut vec: Vec<u32> = Vec::with_capacity(s);
    let mut vecff64: Vec<ff64> = Vec::with_capacity(s);

    for _ in 0..s {
        vec.push(rand::random())
    }

    vec.iter().for_each(|&x| vecff64.push(ff64!(x as f64)));

    vec.sort();
    vecff64.sort();

    for (&x, &xf64) in zip(vec.iter(), vecff64.iter()) {
        assert_eq!(x, f64::from(xf64) as u32);
    }
}

#[test]
fn sort_vec_size_10() {
    sort_vec_wrapper(10);
}

#[test]
fn sort_vec_size_20() {
    sort_vec_wrapper(20);
}

#[test]
fn sort_vec_size_50() {
    sort_vec_wrapper(50);
}

#[test]
fn sort_vec_size_100() {
    sort_vec_wrapper(100);
}

#[test]
fn sort_vec_size_200() {
    sort_vec_wrapper(200);
}

#[test]
fn sort_vec_size_500() {
    sort_vec_wrapper(500);
}
