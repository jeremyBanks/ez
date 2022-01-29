#![doc = include_str!("../README.md")]

pub(crate) use ez_internal as macros;

#[doc(no_inline)]
pub use crate::{errors::*, float::*, int::*, main::*};

pub mod main {
    #![doc = include_str!("./main.md")]

    pub use crate::macros::main;
}

pub mod int {
    #![doc = include_str!("./int.md")]

    #[derive(crate::macros::Int)]
    pub struct Int(i128);

    pub fn int(_: impl std::any::Any) -> Int {
        Int(0)
    }
}

pub mod float {
    #![doc = include_str!("./float.md")]

    #[derive(crate::macros::Float)]
    pub struct Float(f64);

    pub fn float(_: impl std::any::Any) -> Float {
        Float(0.0)
    }
}

pub mod errors {
    #![doc = include_str!("./errors.md")]

    pub use ez_internal::throws;
}
