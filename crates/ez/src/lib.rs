#![doc = include_str!("../README.md")]
#[doc(hidden)]
#[doc = include_str!("./internal.md")]
pub use ez_internal as internal;
#[doc(hidden)]
#[doc = include_str!("./internal.md")]
pub use ez_proc_macro as internal_proc_macro;
#[doc(inline)]
#[doc = include_str!("./error.md")]
///
/// ---
pub use internal::deps::eyre::Report as Error;
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
