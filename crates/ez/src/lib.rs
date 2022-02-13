#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use ez_internal::internal as __;
use ez_internal::publish;

publish! {
    pub use ez_internal::throw;
    prose from "throw.md";
    failing example throw;
}

publish! {
    pub use ez_proc_macro::main;
    prose from "main.md";
    example main_noop;
    example home;
    example tokio;
}

publish! {
    pub use ez_proc_macro::try_throws;
    prose from "try_throws.md";
    failing example try_throws;
}

publish! {
    pub use ez_proc_macro::throws;
    prose from "throws.md";
}

/// A dynamic error reporting type. This is a re-export of
/// [`eyre::Report`](https://docs.rs/eyre/latest/eyre/struct.Report.html) from
/// [the `eyre` package](https://docs.rs/eyre).
///
/// ---
#[doc(inline)]
pub use __::eyre::Report as Error;

/// You may `use ez::prelude::*;`
pub mod prelude {
    #[doc(no_inline)]
    pub use ezio::prelude::*;

    #[doc(no_inline)]
    pub use crate::{io as ezio, main, throw, throws, try_throws};
}

/// This is a re-export of [the `ezio` package](https://docs.rs/ezio).
///
/// ---
#[doc(inline)]
pub use __::ezio as io;
