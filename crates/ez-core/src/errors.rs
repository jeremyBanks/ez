use std::{convert::Infallible, panic::panic_any};

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
