//! A subset of `ez`, for use internally in implementing other parts of `ez`.
//!
//! This package must be imported as `ez` for macro compatibility.

#![warn(unused_crate_dependencies)]

mod errors;

pub use {
    eyre::{Report as Error, WrapErr},
    ez_macro_rules::throw,
    ez_proc_macro::{main, throws, try_throws},
};

pub mod __ {
    pub use {
        crate::errors::ErrorPanicker,
        std::result::Result::{self, Err, Ok},
    };
}
