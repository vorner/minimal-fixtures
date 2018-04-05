#![feature(proc_macro)]

extern crate minimal_fixtures_macros;

use std::iter::{self, Once};

pub use minimal_fixtures_macros::minimal_fixture;

pub trait Fixture : Clone + Sized {
    type It: Iterator<Item = Self>;
    fn values() -> Self::It;
}

impl<D: Clone + Default> Fixture for D {
    type It = Once<D>;
    fn values() -> Self::It {
        iter::once(Default::default())
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[minimal_fixture]
    fn do_test() {
        assert!(true);
    }
}
*/
