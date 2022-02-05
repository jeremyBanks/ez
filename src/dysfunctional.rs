#![doc(hidden)]
//! Dysfunctional stub implementations of some traits.

use std::{panic::panic_any, convert::Infallible};

#[derive(Debug, Clone, Copy)]
/// Drops an iterator without consuming any elements.
pub struct IteratorDropper;
impl<Item> FromIterator<Item> for IteratorDropper {
    fn from_iter<Iterator: IntoIterator<Item = Item>>(_: Iterator) -> Self {
        IteratorDropper
    }
}


/// An uninhabited pseudo-Error-type that panics when any other error types to
/// convert into it.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ErrorPanicker(Infallible);

#[allow(clippy::fallible_impl_from)]
impl<T> From<T> for ErrorPanicker
where
    T: Into<eyre::Report>,
{
    fn from(t: T) -> Self {
        panic_any(t.into())
    }
}
