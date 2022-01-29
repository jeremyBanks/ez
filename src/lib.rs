#![doc = include_str!("../README.md")]

pub(crate) use ez_internal as macros;

#[doc(no_inline)]
pub use crate::{aliases::*, errors::*, float::*, int::*, main::*};

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

pub mod aliases {
    //! Aliases for built-in functionality, using names that may be more
    //! recognizable to developers coming from other languages.

    use std::collections::{HashMap, HashSet};

    /// Collects values from an iterator into a [`Vec`].
    ///
    /// This is equivalent to `Vec::from_iter(x)`.
    pub fn vec<T>(x: impl IntoIterator<Item = T>) -> Vec<T> {
        Vec::from_iter(x)
    }

    /// Collects values from an iterator into a [`HashSet`].
    ///
    /// This is equivalent to `HashSet::from_iter(x)`.
    pub fn set<T: std::cmp::Eq + std::hash::Hash>(x: impl IntoIterator<Item = T>) -> HashSet<T> {
        HashSet::from_iter(x)
    }

    /// Collects `(key, value)` pairs from an iterator into a [`HashMap`].
    ///
    /// This is equivalent to `HashMap::from_iter(x)`.
    pub fn map<K: std::cmp::Eq + std::hash::Hash, V>(
        x: impl IntoIterator<Item = (K, V)>,
    ) -> HashMap<K, V> {
        HashMap::from_iter(x)
    }

    /// Returns a clone of the referenced value.
    ///
    /// This is equivalent to `x.clone()`.
    pub fn clone<T: Clone>(x: &T) -> T {
        x.clone()
    }

    /// If you're coming from Python, you should know that Rust only has one
    /// typing of cloning built-in, via the `.clone()` method on compatible
    /// types. The clones are usually deep, but the behaviour can vary by type.
    ///
    /// This is an alias of our [`clone()`] function, which is equivalent to
    /// `x.clone()`.
    #[deprecated = "`clone` is the preferred name for this function"]
    pub fn deep_clone<T: Clone>(x: &T) -> T {
        clone(x)
    }

    /// If you're coming from Python, you should know that what Python calls a
    /// `list`, Rust calls a [`Vec`].
    ///
    /// This is an alias of our [`vec()`] function, which is equivalent to
    /// `Vec::from_iter(x)`.
    #[deprecated = "`vec` is the preferred name for this function"]
    pub fn list<T>(x: impl IntoIterator<Item = T>) -> Vec<T> {
        vec(x)
    }

    /// If you're coming from Python, you should know that what Python calls a
    /// `dict`, Rust calls a [`HashMap`].
    ///
    /// This is an alias of our [`map()`] function, which is equivalent to
    /// `HashMap::from_iter(x)`.
    #[deprecated = "`map` is the preferred name for this function"]
    pub fn dict<K: std::cmp::Eq + std::hash::Hash, V>(
        x: impl IntoIterator<Item = (K, V)>,
    ) -> HashMap<K, V> {
        map(x)
    }

    /// If you're coming from a dynamic language like JavaScript or Ruby, you
    /// should know that what they call a `Array`, Rust calls a [`Vec`].
    ///
    /// This is an alias of our [`vec()`] function. Rust does have [something it
    /// calls "arrays"][ARRAY], but their length is fixed, like C arrays.
    ///
    /// [ARRAY]: https://doc.rust-lang.org/std/primitive.array.html
    #[deprecated = "`vec` is the preferred name for this function"]
    pub fn array<T>(x: impl IntoIterator<Item = T>) -> Vec<T> {
        vec(x)
    }
}

#[allow(unused)]
#[cfg(test)]
#[test]
fn test() {
    let x = map(list(set([(1, 2), (1, 3), (1, 2)])));
}
