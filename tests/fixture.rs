#![feature(proc_macro)]
extern crate minimal_fixtures;

use minimal_fixtures::minimal_fixture;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
struct X;

/*
#[minimal_fixture]
fn do_test(x: X) {
    assert_eq!(x, X);
}
*/

#[minimal_fixture]
fn do_double_test(x1: X, x2: X) {
    assert_eq!(x1, x2);
}
