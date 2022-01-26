#![feature(doc_cfg)]
#![doc = include_str!("../README.md")]

use std::convert::Infallible as never;

pub fn int(_: never) -> Int {
    todo!()
}

pub struct Int(never);

#[cfg(any(feature = "unstable-float", doc))]
#[doc(cfg(feature = "unstable-float"))]
pub fn float(_: never) -> Float {
    todo!()
}

#[cfg(any(feature = "unstable-float", doc))]
#[doc(cfg(feature = "unstable-float"))]
pub struct Float(never);
