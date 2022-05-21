use {
    doop::{doop, dooped},
    std::{
        fmt::{Debug, Display},
        marker::PhantomData,
        ops::Add,
    },
};

#[test]
fn main() -> Result<(), eyre::Report> {
    let x = int(2);

    dbg!(Int::MAX + Int::MAX);
    dbg!(Int::MIN + Int::MAX);
    dbg!(Int::MIN + Int::MIN);

    Ok(())
}

pub fn int(value: i128) -> Int<Infer> {
    value.into()
}

pub trait ErrorMode {}

doop! {
    let ERROR_MODES = [Infer, Panicking, Checked, Saturating, Wrapping];

    for ERROR_MODE in ERROR_MODES {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash, Ord, PartialOrd)]
        pub struct ERROR_MODE;
        impl ErrorMode for ERROR_MODE {}
    }
}

type DefaultErrorMode = Panicking;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Int<ErrorMode: self::ErrorMode = Infer> {
    value: i128,
    error_mode: PhantomData<ErrorMode>,
}

impl<ErrorMode: self::ErrorMode> Int<ErrorMode> {
    const fn new(value: i128) -> Self {
        Self { value, error_mode: PhantomData }
    }
    pub const MAX: Self = Self::new(i128::MAX);
    pub const MIN: Self = Self::new(i128::MIN);

    pub fn checked(self) -> Int<Checked> {
        self.value.into()
    }

    pub fn panicking(self) -> Int<Panicking> {
        self.value.into()
    }

    pub fn saturating(self) -> Int<Saturating> {
        self.value.into()
    }

    pub fn wrapping(self) -> Int<Wrapping> {
        self.value.into()
    }
}

impl<ErrorMode: self::ErrorMode> Debug for Int<ErrorMode> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

impl<ErrorMode: self::ErrorMode> Display for Int<ErrorMode> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl<ErrorMode: self::ErrorMode> From<i128> for Int<ErrorMode> {
    fn from(value: i128) -> Self {
        Self { value, error_mode: PhantomData }
    }
}

#[derive(thiserror::Error)]
#[error(transparent)]
pub struct IntError(eyre::Report);
impl Debug for IntError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
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

pub type CheckedInt = Int<Checked>;
pub type PanickingInt = Int<Panicking>;
pub type InferInt = Int<Infer>;

type IntResult<T> = Result<Int<T>, IntError>;
type CheckedIntResult = IntResult<Checked>;
type PanickingIntResult = IntResult<Panicking>;
type InferIntResult = IntResult<Infer>;

doop! {
    for (SelfType, OtherType, ResultType, self_value, other_value, result_value) in [
        (CheckedInt, CheckedInt, CheckedIntResult, self, rhs, Ok(CheckedInt::from(value))),
        (CheckedInt, i128, CheckedIntResult, self, CheckedInt::from(rhs), Ok(CheckedInt::from(value))),
        (i128, CheckedInt, CheckedIntResult, CheckedInt::from(self), rhs, Ok(CheckedInt::from(value))),
        (CheckedIntResult, CheckedInt, CheckedIntResult, self?, rhs, Ok(CheckedInt::from(value))),
        (CheckedInt, CheckedIntResult, CheckedIntResult, self, rhs?, Ok(CheckedInt::from(value))),
        // (CheckedIntResult, CheckedIntResult, CheckedIntResult, self?, rhs?, Ok(CheckedInt::from(value))),
    ] {
        impl Add<OtherType> for SelfType {
            type Output = ResultType;

            fn add(self, rhs: OtherType) -> ResultType {
                match self_value.value.checked_add(other_value.value) {
                    Some(value) => result_value,
                    None => Err(IntError::overflow()),
                }
            }
        }
    }
}
