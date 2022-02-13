#![doc = include_str!("../README.md")]
pub mod prelude {
    //! You may `use ez::prelude::*;`
    #[doc(no_inline)]
    pub use ezio::prelude::*;

    #[doc(no_inline)]
    pub use crate::{io as ezio, main, throw, throws, try_throws};
}
#[doc(hidden)]
pub use ez_internal::internal as __;
#[doc(inline)]
#[doc = include_str!("./throw.md")]
pub use ez_internal::throw;
#[doc(inline)]
#[doc = include_str!("./main.md")]
pub use ez_proc_macro::main;
#[doc(hidden)]
#[doc = include_str!("./try_throws.md")]
pub use ez_proc_macro::throws as try_throws;
#[doc(inline)]
#[doc = include_str!("./throws.md")]
pub use ez_proc_macro::throws;
#[doc(inline)]
/// A dynamic error reporting type. This is a re-export of [`eyre::Report`](https://docs.rs/eyre/latest/eyre/struct.Report.html) from [the `eyre` package](https://docs.rs/eyre).
///
/// ---
pub use __::eyre::Report as Error;
#[doc(inline)]
/// This is a re-export of [the `ezio` package](https://docs.rs/ezio).
///
/// ---
pub use __::ezio as io;
