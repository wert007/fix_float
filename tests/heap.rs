use fix_float::*;
use std::collections::BinaryHeap;

#[test]
fn simple() {
    let mut heap: BinaryHeap<ff64> = BinaryHeap::new();

    heap.push(ff64!(5.0));
    heap.push(ff64!(8.0));
    heap.push(ff64!(3.0));
    heap.push(ff64!(3.0));
    heap.push(ff64!(1.0));
    heap.push(ff64!(9.0));
    heap.push(ff64!(0.0));
    heap.push(ff64!(-0.0));

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
