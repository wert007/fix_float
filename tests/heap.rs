use fix_float::*;

use rand::{self, prelude::SliceRandom};
use std::collections::BinaryHeap;

#[test]
fn simple() {
    let mut heap: BinaryHeap<ff64> = BinaryHeap::new();

    heap.push(ff64!(0.0));
    heap.push(ff64!(5.0));
    heap.push(ff64!(8.0));
    heap.push(ff64!(3.0));
    heap.push(ff64!(-0.0));
    heap.push(ff64!(3.0));
    heap.push(ff64!(1.0));
    heap.push(ff64!(9.0));

    assert_eq!(heap.pop(), Some(ff64!(9.0)));
    assert_eq!(heap.pop(), Some(ff64!(8.0)));
    assert_eq!(heap.pop(), Some(ff64!(5.0)));
    assert_eq!(heap.pop(), Some(ff64!(3.0)));
    assert_eq!(heap.pop(), Some(ff64!(3.0)));
    assert_eq!(heap.pop(), Some(ff64!(1.0)));
    assert_eq!(heap.pop(), Some(ff64!(0.0)));
    assert_eq!(heap.pop(), Some(ff64!(0.0)));
    assert_eq!(heap.pop(), None);
}

#[test]
fn range() {
    let mut heap: BinaryHeap<ff64> = BinaryHeap::new();

    let mut vec = Vec::new();

    for i in 0..=1000 {
        vec.push(i as f64);
    }

    vec.shuffle(&mut rand::thread_rng());

    for elem in vec {
        heap.push(ff64!(elem));
    }

    let mut index = 1000;

    while index >= 0 {
        assert_eq!(*heap.pop().unwrap(), index as f64);
        index -= 1;
    }

    assert_eq!(heap.pop(), None)
}
