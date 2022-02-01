#![doc = include_str!("../README.md")]

#[doc(no_inline)]
pub use crate::{errors::*, main::*};

pub mod main {
    #![doc = include_str!("./main.md")]

    #[doc(inline)]
    /// Applied to a main (entry point) function with a bunch of sugar.
    pub use ez_internal::main;
}

pub mod errors {
    #![doc = include_str!("./errors.md")]

    #[doc(inline)]
    /// Applied to the function to allow use of `?` operator in the body, but
    /// any errors will panic (crash) the program.
    pub use ez_internal::panics;
    #[doc(inline)]
    /// Applied to a function to return a [`Result`], with an [`eyre::Report`]
    /// by default.
    pub use ez_internal::throws;
    #[doc(inline)]
    /// Applied to a function to create two versions, one that panics on error
    /// (as [`panics`]), and a copy with `try_` prefixed to the name which
    /// returns a [`Result`] instead.
    pub use ez_internal::try_or_panics;
}
