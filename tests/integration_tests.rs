// import setup
mod common;

#[macro_use] // <-- import the convenience macro (optional)
extern crate assert_cli;

#[test]
fn test_() {
    // using common code.
    let _logger = common::setup();

    assert_eq!(1,1);
}