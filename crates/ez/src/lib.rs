#![doc = include_str!("../README.md")]

pub mod prelude {
    #[doc(inline)]
    pub use ::ezio::prelude::*;
}

#[doc(inline)]
pub use ::{
    ez_core::*,
    ez_int::*,
    ezio::{file, random, stdio, string},
    noisy_float::types::R64 as Float,
};
