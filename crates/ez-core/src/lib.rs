mod errors;
mod main;

pub use {
    eyre::{Report as Error, WrapErr},
    ez_macro_rules::throw,
    ez_proc_macro::{main, throws, try_throws},
};

pub mod __ {
    //! **⚠️ INTERNAL! DO NOT USE!**
    //!
    //! This should not be considered part of this crate's public API.
    //!
    //! This is public only due to internal implementation requirements
    pub use {
        crate::{
            errors::ErrorPanicker,
            main::{entry_point, IteratorDropper},
        },
        std::{
            option::Option::{self, None, Some},
            result::Result::{self, Err, Ok},
        },
        tokio, tracing,
    };
}
