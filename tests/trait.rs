use fix_float::*;

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
