pub extern crate alloc;
pub extern crate core;
pub extern crate std;

mod dysfunctional;
mod macro_rules;
#[expect(special_module_name, reason = "main.rs is used as a module for entry_point function")]
mod main;
pub mod proc_macros;

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
