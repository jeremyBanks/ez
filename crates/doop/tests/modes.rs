use std::{
    marker::PhantomData,
    ops::{Add, Div},
};

#[test]
fn main() {
    let one = CheckedInt::from(1);
    let zero = CheckedInt::from(0);

    let two = one + one;
    let three = two + one + zero;

    eprintln!("zero = {zero:?}");
    eprintln!("one = {one:?}");
    eprintln!("two = {two:?}");
    eprintln!("three = {three:?}");
}

pub mod error_mode {
    pub trait ErrorMode: std::fmt::Debug {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash, Ord, PartialOrd)]
    pub struct Default;
    impl ErrorMode for Default {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash, Ord, PartialOrd)]
    pub struct Panicky;
    impl ErrorMode for Panicky {}

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash, Ord, PartialOrd)]
    pub struct Checked;
    impl ErrorMode for Checked {}
}
use error_mode::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Int<ErrorMode: self::ErrorMode = error_mode::Default> {
    value: i128,
    error_mode: PhantomData<ErrorMode>,
}

impl<ErrorMode: self::ErrorMode> From<i128> for Int<ErrorMode> {
    fn from(value: i128) -> Self {
        Self { value, error_mode: PhantomData }
    }
}

#[derive(thiserror::Error)]
#[error(transparent)]
pub struct IntError(eyre::Report);
impl std::fmt::Debug for IntError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl IntError {
    pub(crate) fn msg(msg: &'static str) -> Self {
        Self(eyre::Report::msg(msg))
    }

    pub(crate) fn overflow() -> Self {
        Self::msg("operation overflowed (the result was too large to fit)")
    }

    pub(crate) fn divided_by_zero() -> Self {
        Self::msg("divided by zero")
    }
}

pub type CheckedInt = Int<error_mode::Checked>;
pub type PanickyInt = Int<error_mode::Panicky>;
pub type DefaultInt = Int<error_mode::Default>;

impl Add<CheckedInt> for CheckedInt {
    type Output = Result<CheckedInt, IntError>;

    fn add(self, rhs: CheckedInt) -> Self::Output {
        match self.value.checked_add(rhs.value) {
            Some(value) => Ok(Self::from(value)),
            None => Err(IntError::overflow()),
        }
    }
}

impl Div<CheckedInt> for CheckedInt {
    type Output = Result<CheckedInt, IntError>;

    fn div(self, rhs: CheckedInt) -> Self::Output {
        match self.value.checked_div(rhs.value) {
            Some(value) => Ok(Self::from(value)),
            None => Err(IntError::divided_by_zero()),
        }
    }
}
