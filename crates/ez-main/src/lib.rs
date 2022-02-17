#![warn(unused_crate_dependencies)]

mod main;

pub mod __ {
    pub use {
        crate::main::{entry_point, IteratorDropper},
        tokio, tracing,
    };
}
