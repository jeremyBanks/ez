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
            alloc,
            core::{
                self,
                option::Option::{self, None, Some},
                result::Result::{self, Err, Ok},
            },
            dysfunctional::{ErrorPanicker, IteratorDropper},
            main::run,
            std,
        },
        eyre, ezio, fehler, proc_macro2, quote, syn, tokio, tracing,
    };
}

#[macro_export]
macro_rules! publish {
    {
        use $path:path;
        docs from $doc:ident;
        $(include example $example:ident;)*
        $(failing example $failing:ident;)*
        $(panicking example $panicking:ident;)*
    } => {
        #[doc = include_str!(concat!("./", stringify!($doc), ".md"))]
        $(
        ///
        #[doc = concat!("## Example `", stringify!($example), "`")]
        ///
        /// ```rust
        /// # use ez::__::*;
        #[doc = include_str!(concat!("../examples/", stringify!($example), ".rs"))]
        /// ```
        )*
        $(
        ///
        #[doc = concat!("## Example `", stringify!($failing), "`")]
        ///
        /// ```should_panic
        /// # use ez::__::*;
        #[doc = include_str!(concat!("../examples/", stringify!($failing), ".rs"))]
        /// ```
        )*
        ///
        #[doc(inline)]
        pub use $path;
    }
}
