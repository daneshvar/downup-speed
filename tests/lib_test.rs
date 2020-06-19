extern crate downup;

mod common;

#[test]
fn test_add() {
    common::setup();
    assert_eq!(downup::add(3, 2), 5);
}
