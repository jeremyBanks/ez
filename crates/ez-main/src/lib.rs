#![warn(unused_crate_dependencies)]

mod main;

#[doc(hidden)]
pub use ez_macros::proc::ly;
pub use ez_macros::proc::main;

pub mod __ {
    pub use {
        crate::main::{entry_point, IteratorDropper},
        tokio, tracing,
    };
}
