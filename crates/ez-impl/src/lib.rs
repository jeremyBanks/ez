pub extern crate alloc;
pub extern crate core;
pub extern crate std;

mod dysfunctional;
mod macro_rules;
mod main;
pub mod proc_macros;

pub use crate::macro_rules::*;

pub mod internal {
    //! **⚠️ INTERNAL! DO NOT USE!**
    //!
    //! This should not be considered part of this crate's public API.
    //!
    //! This is public only due to internal implementation requirements
    pub use {
        crate::{
            core::{
                option::Option::{self, None, Some},
                result::Result::{self, Err, Ok},
            },
            dysfunctional::{ErrorPanicker, IteratorDropper},
            main::entry_point,
        },
        tokio, tracing,
    };
}
