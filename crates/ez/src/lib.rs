#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use ez_impl::internal as __;
use ez_impl::publish;

publish! {
    pub use ez_impl::throw;
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

/// (_[eyre::Report](https://docs.rs/eyre/latest/eyre/struct.Report.html)_) A dynamic error reporting type.
///
/// ---
#[doc(inline)]
pub use eyre::Report as Error;
#[cfg(feature = "ezio")]
/// (_[ezio::random](https://docs.rs/ezio/latest/ezio/random)_)
#[doc(inline)]
pub use ezio::random;
