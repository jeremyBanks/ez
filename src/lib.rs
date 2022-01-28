#![feature(doc_cfg)]
#![doc = include_str!("../README.md")]

pub(crate) use ez_internal as internal;

pub mod int;
mod traits;

#[doc(inline)]
/// A re-export of the (unaffiliated) [`::ezio`] crate.
///
/// ---
pub use ezio as io;

#[doc(inline)]
pub use crate::traits::*;

/// Re-exports the most widely-useful features from this crate.
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::io::prelude::*;
}
