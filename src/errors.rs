//! Simplified error propagation.

///
pub use _proc_macros::panics;
///
pub use _proc_macros::throws;

/// Returns from the enclosing function with a [`Result::Err`].
///
/// This will contain either a provided error object, or an error message
/// (if the first argument is a formatting string literal). Error messages
/// are formatted into [`eyre::Report`]s using [`eyre::eyre!`], so you'll
/// get an error if you try to use an error message in a function that
/// expects a more-specific error type in its [`Result`].
#[macro_export]
macro_rules! throw {
    ($msg:literal $(,)?) => {
        $crate::deps::fehler::throw!(eyre::Report::msg($msg));
    };

    ($msg:literal $(, $rest:tt)* $(,)?) => {
        $crate::deps::fehler::throw!(eyre::Report::msg(format!($msg $(, $rest)*)));
    };

    ($error:expr $(,)?) => {
        $crate::deps::fehler::throw!($error);
    };

    ($(,)?) => {
        $crate::deps::fehler::throw!();
    };
}

pub use crate::throw;
