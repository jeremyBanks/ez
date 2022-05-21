use ::{
    core::{
        clone::Clone,
        cmp::{Eq, Ord, PartialEq, PartialOrd},
        default::Default,
        fmt::{Debug, Display},
        hash::Hash,
        marker::{Copy, PhantomData},
        ops::{Add, Div, Mul, Sub},
    },
    doop::doop,
    num_traits::PrimInt,
    thiserror::Error,
    paste::paste,
};

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Default, Hash, Copy)]
pub struct Int<M: ErrorMode = Infer> {
    value: i128,
    meta: PhantomData<*mut (ErrorMode)>,
}

#[derive(Error, Debug)]
pub enum IntError {
    #[error("result overflowed when adding: `{0} + {1}`")]
    AddOverflow(Int, Int),

    #[error("result overflowed when subtracting: `{0} - {1}`")]
    SubOverflow(Int, Int),

    #[error("result overflowed when multiplying: `{0} * {1}`")]
    MulOverflow(Int, Int),

    #[error("divided by zero: `{0} / 0`")]
    DivisionByZero(Int),
}

impl<M: ErrorMode> Int<M> {
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

    fn try_add(self, other: impl Into<Self>) -> Result<Self, IntError> {
        match self.value.checked_add(other.into().value) {
            Some(value) => Ok(Self::new(value)),
            None => Err(IntError::AddOverflow(self, other)),
        }
    }

    fn try_sub(self, other: impl Into<Self>) -> Result<Self, IntError> {
        match self.value.checked_sub(other.into().value) {
            Some(value) => Ok(Self::new(value)),
            None => Err(IntError::SubOverflow(self, other)),
        }
    }

    fn try_mul(self, other: impl Into<Self>) -> Result<Self, IntError> {
        match self.value.checked_mul(other.into().value) {
            Some(value) => Ok(Self::new(value)),
            None => Err(IntError::AddOverflow(self, other)),
        }
    }

    fn try_div(self, other: impl Into<Self>) -> Result<Self, IntError> {
        match self.value.checked_div(other.into().value) {
            Some(value) => Ok(Self::new(value)),
            None => Err(IntError::DivisionByZero(self)),
        }
    }
}

impl Add<Int<Checked>> for Int<Checked> {
    type Output = Int<Checked>;

    fn add(self, rhs: Int<Checked>) -> Self::Output {
        let result = self.value.checked_add(rhs.value);
        match result {
            Some(value) => Ok(Int::new(value)),
            None => Err(IntError::AddOverflow(self, rhs)),
        }
    }
}

pub trait ErrorMode {}
type DefaultErrorMode = Panicking;

doop! {
    let ERROR_MODE_LIST = [
        (Infer, unwrap()),
        (Panicking, unwrap()),
        (Checked, unwrap()),
    ];

    for ERROR_MODE in ERROR_MODE_LIST {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash, Ord, PartialOrd)]
        pub struct ERROR_MODE;
        impl ErrorMode for ERROR_MODE {}
    }


}

#[test]
fn test() -> Result<(), eyre::Report> {
    let two = int(2).checked();

    // let x = two;
    // let y = x + two;
    // let z = y + two;

    // let x = two.panicking();
    // let y = x + two;
    // let z = y + two;

    // let x = two.checked();
    // let y = x + two;
    // let z = y + two;

    // dbg!(Int::MAX + Int::MAX);
    // dbg!(Int::MIN + Int::MAX);
    // dbg!(Int::MIN + Int::MIN);

    Ok(())
}
