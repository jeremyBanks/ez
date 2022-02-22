#![doc = include_str!("../README.md")]

#[doc(inline)]
pub use ::ez_core::{main, throw, throws, try_throws, Error};
#[cfg(feature = "ez-int")]
pub use ::ez_int::{int, Int};
#[cfg(feature = "ezio")]
pub use ::ezio::{file, random, stdio, string};

pub mod prelude {
    #[cfg(feature = "ez-batteries")]
    #[doc(inline)]
    pub use ::ez_batteries::prelude::*;
    #[cfg(feature = "ezio")]
    #[doc(inline)]
    pub use ::ezio::prelude::*;
}

#[cfg(feature = "ez-batteries")]
pub use ::ez_batteries::batteries;

#[doc(hidden)]
pub mod __ {
    //! **⚠️ INTERNAL! DO NOT USE!**
    //!
    //! This should not be considered part of this crate's public API.
    //!
    //! This is public only due to internal implementation requirements
    #[doc(inline)]
    pub use ::ez_core::__::*;
    #[cfg(feature = "ez-main")]
    #[doc(inline)]
    pub use ::ez_main::__::*;
}
