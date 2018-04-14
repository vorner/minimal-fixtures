#![feature(proc_macro)]

extern crate minimal_fixtures_macros;

use std::iter::{self, Once};

pub use minimal_fixtures_macros::minimal_fixture;

/// The type can be used as a fixture-parameter for a test.
///
/// Such type is required to be able to produce a serie of values the test function will be run
/// with. These are produced by providing an iterator.
///
/// As a convenience, it is auto-implemented for types implementing `Default` (and produces just
/// one value).
pub trait Fixture : Clone + Sized {
    /// The type of iterator returned.
    type It: Iterator<Item = Self>;
    /// Produces the actual iterator.
    fn values() -> Self::It;
}

impl<D: Clone + Default> Fixture for D {
    type It = Once<D>;
    fn values() -> Self::It {
        iter::once(Default::default())
    }
}
