fn main() {}

mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;

pub trait ErrorMode: sealed::Sealed {}

pub struct Panics;
impl self::Sealed for Panics {}
impl ErrorMode for Panics {}

pub struct Results;
impl self::Sealed for Results {}
impl ErrorMode for Results {}

pub struct MyAddableThing<ErrorMode: self::ErrorMode = Panics> {
    error_mode: ErrorMode,
    inner: i128,
}
