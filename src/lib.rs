#![doc = include_str!("../README.md")]

pub(crate) use ez_internal as internal;

#[doc(inline)]
pub use crate::{errors::*, float::*, int::*, vec::*};

pub mod prelude {}

pub mod int {
    #![doc = include_str!("./int.md")]

    #[derive(crate::internal::Int)]
    pub struct Int(i128);

    pub fn int(_: impl std::any::Any) -> Int {
        Int(0)
    }
}

pub mod float {
    pub struct Float(f64);

    pub fn float(_: impl std::any::Any) -> Float {
        Float(0.0)
    }
}

pub mod errors {}

pub mod vec {
    /// Collects an iterator into a [`Vec`].
    pub fn vec<T>(x: impl IntoIterator<Item = T>) -> Vec<T> {
        Vec::from_iter(x)
    }

    /// If you're coming from Python, you should know that what Python calls a
    /// `list`, Rust calls a [`Vec`]. This is an alias of our [`vec()`]
    /// function.
    #[deprecated = "`vec` is the preferred name for this function"]
    #[doc(hidden)]
    pub fn list<T>(x: impl IntoIterator<Item = T>) -> Vec<T> {
        vec(x)
    }
}

#[allow(unused)]
#[cfg(test)]
#[test]
fn test() {
    let x = list([2]);
}
