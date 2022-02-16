#![doc = include_str!("../README.md")]

#[doc(inline)]
pub use ::{
    ez_core::{main, throw, throws, try_throws, Error},
    ez_int::{int, Int},
    ezio::{file, random, stdio, string},
};

pub mod prelude {
    #[doc(inline)]
    pub use ::ezio::prelude::*;
}

#[doc(hidden)]
pub mod __ {
    //! **⚠️ INTERNAL! DO NOT USE!**
    //!
    //! This should not be considered part of this crate's public API.
    //!
    //! This is public only due to internal implementation requirements
    pub use ::{ez_core::__::*, ez_main::__::*};
}
