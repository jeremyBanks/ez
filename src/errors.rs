//! Simplified error propagation.

use std::{convert::Infallible, fmt::Debug};

///
pub use ez_internal::panics;
///
pub use ez_internal::throws;

/// An uninhabited pseudo-Error-type that panics when any other error types to
/// convert into it.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Panic(Infallible);

#[allow(clippy::fallible_impl_from)]
impl<T> From<T> for Panic
where
    T: Into<eyre::Report>,
{
    fn from(t: T) -> Self {
        panic!("{:?}", eyre::eyre!(t));
    }
}

/// Returns from the enclosing function with a [`Result::Err`].
///
/// This will contain either a provided error object, or an error message
/// (if the first argument is a formatting string literal). Error messages
/// are formatted into [`eyre::Report`]s using [`eyre::eyre!`], so you'll
/// get an error if you try to use an error message in a function that
/// expects a more-specific error type in its [`Result`].
#[macro_export]
macro_rules! throw {
    ($msg:literal $(, $rest:tt)* $(,)?) => {
        $crate::deps::fehler::throw!(::eyre::eyre!($msg $(, $rest)*));
    };

    ($error:expr) => {
        $crate::deps::fehler::throw!($error);
    };

    () => {
        $crate::deps::fehler::throw!();
    };
}

pub use crate::throw;
