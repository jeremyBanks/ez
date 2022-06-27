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
    meta: PhantomData<ErrorMode>,
}

impl<A: ErrorMode, B: PrimInt> Int<A, B> {
    const fn new(value: i128) -> Self {
        Self { value, meta: PhantomData }
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

impl<AnyErrorMode: ErrorMode> Debug for Int<AnyErrorMode>{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

impl<AnyErrorMode: ErrorMode> Display for Int<AnyErrorMode> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl<AnyErrorMode: ErrorMode> From<i128> Int<AnyErrorMode> {
    fn from(value: i128) -> Self {
        Self { value, error_mode: PhantomData }
    }
}

#[derive(Error, Debug)]
pub enum IntError {
    #[error("operation overflowed (the result was out-of-bounds")]
    Overflow(eyre::Report),

    #[error("division by zero (no result exists)")]
    DivisionByZero(eyre::Report),
}

impl IntError {
    fn overflow() -> IntError {
        IntError::Overflow(eyre::Report::msg("operation overflowed (the result was out-of-bounds"))
    }

    fn division_by_zero() -> IntError {
        IntError::DivisionByZero(eyre::Report::msg("division by zero (no result exists)"))
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
