#![doc = include_str!("../README.md")]
pub mod prelude {
    //! `use ez::prelude::*` if you'd like, but note that
    //! [`#[ez::main]`][crate::main] isn't included here because [importing
    //! it directly conflicts with a built-in `#[main]` attribute](https://stackoverflow.com/q/71024443/1114).
    pub use crate::{internal::deps::ezio::prelude::*, panics, throw, throws, try_throws};
}
#[doc(hidden)]
#[doc = include_str!("./internal.md")]
pub use ez_internal as internal;
#[doc(hidden)]
#[doc = include_str!("./internal.md")]
pub use ez_proc_macro as internal_proc_macro;
#[doc(inline)]
pub use internal::deps::ezio as io;
#[allow(unused_imports)]
use internal::deps::*;
#[doc(inline)]
#[doc = include_str!("./throw.md")]
pub use internal::throw;
#[doc(inline)]
#[doc = include_str!("./main.md")]
pub use internal_proc_macro::main;
#[doc(inline)]
#[doc = include_str!("./panics.md")]
pub use internal_proc_macro::panics;
#[doc(inline)]
#[doc = include_str!("./try_throws.md")]
pub use internal_proc_macro::throws as try_throws;
#[doc(inline)]
#[doc = include_str!("./throws.md")]
pub use internal_proc_macro::throws;
