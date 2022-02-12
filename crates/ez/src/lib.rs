#![doc = include_str!("../README.md")]
pub mod prelude {
    //! You may `use ez::prelude::*;`
    pub use crate::{throw, throws, try_throws, __::ezio::prelude::*};
}
#[doc(hidden)]
use ez_internal as internal;
#[doc(hidden)]
use ez_proc_macro as internal_proc_macro;
#[doc(hidden)]
pub use internal::exposed as __;
#[doc(inline)]
#[doc = include_str!("./throw.md")]
pub use internal::throw;
#[doc(inline)]
#[doc = include_str!("./main.md")]
pub use internal_proc_macro::main;
#[doc(inline)]
#[doc = include_str!("./try_throws.md")]
pub use internal_proc_macro::throws as try_throws;
#[doc(inline)]
#[doc = include_str!("./throws.md")]
pub use internal_proc_macro::throws;
#[doc(inline)]
/// A dynamic error type. This is a re-export of [`eyre::Report`](https://docs.rs/eyre/latest/eyre/struct.Report.html) from [the `eyre` package](https://docs.rs/eyre).
///
/// ---
pub use __::eyre::Report as Error;
#[doc(inline)]
#[doc(inline)]
/// This is a re-export of [the `ezio` package](https://docs.rs/ezio).
///
/// ---
pub use __::ezio as io;
