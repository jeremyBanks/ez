#![doc = include_str!("../README.md")]

#[doc(inline)]
pub use ::ez_core::{throw, throws, try_throws, Error};
#[cfg(feature = "ez-main")]
#[doc(hidden)]
pub use ::ez_main::ly;
#[cfg(feature = "ez-main")]
#[doc(inline)]
pub use ::ez_main::main;
#[cfg(feature = "ezio")]
pub use ::ezio::{file, random, stdio, string};
#[cfg(feature = "implicint")]
pub use ::implicint::{int, Int};

pub mod prelude {
    #[cfg(feature = "batteries")]
    #[doc(inline)]
    pub use ::batteries::prelude::*;
    pub use ::ez_main::main;
    #[cfg(feature = "ezio")]
    #[doc(inline)]
    pub use ::ezio::prelude::*;
}

#[cfg(feature = "batteries")]
pub use ::batteries::batteries;

#[doc(hidden)]
pub mod __ {
    //! **⚠️ INTERNAL! DO NOT USE!**
    //!
    //! This should not be considered part of this crate's public API.
    //!
    //! The contents are public only due to internal implementation
    //! requirements.
    #[doc(inline)]
    pub use ::ez_core::__::*;
    #[cfg(feature = "ez-main")]
    #[doc(inline)]
    pub use ::ez_main::__::*;
}
