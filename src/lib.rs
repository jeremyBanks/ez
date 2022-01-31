#![doc = include_str!("../README.md")]

#[doc(no_inline)]
pub use crate::{errors::*, main::*};

pub mod main {
    #![doc = include_str!("./main.md")]

    #[doc(inline)]
    pub use ez_internal::main;
}

pub mod errors {
    #![doc = include_str!("./errors.md")]

    #[doc(inline)]
    pub use ez_internal::throws;
}
