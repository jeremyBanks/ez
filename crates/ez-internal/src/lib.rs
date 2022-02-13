pub extern crate alloc;
pub extern crate core;
pub extern crate std;

mod dysfunctional;
mod macro_rules;
mod main;
mod proc_macros;

pub use crate::{macro_rules::*, proc_macros::*};

pub mod internal {
    //! **⚠️ INTERNAL! DO NOT USE!**
    //!
    //! This should not be considered part of this crate's public API.
    //!
    //! This is only public due to internal implementation requirements
    pub use {
        crate::{
            alloc, core,
            dysfunctional::{ErrorPanicker, IteratorDropper},
            main::run,
            std,
        },
        eyre, ezio, fehler, proc_macro2, quote, syn, tokio, tracing,
    };
}
