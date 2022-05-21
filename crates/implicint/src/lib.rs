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

    pub fn saturating(self) -> Int<Saturating> {
        self.value.into()
    }

    pub fn wrapping(self) -> Int<Wrapping> {
        self.value.into()
    }
}

pub trait ErrorMode {}
type DefaultErrorMode = Panicking;

doop! {
    let ERROR_MODE_LIST = [
        (Infer),
        (Panicking),
        (Checked),
        (Saturating),
        (Wrapping),
    ];

    let OPERATIONS_LIST_A = [
        (Add, add, checked_add, saturating_add),
        (Sub, sub, checked_sub, saturating_sub),
        (Mul, mul, checked_mul, saturating_mul),
        (Div, div, checked_div, div),
    ];

    let OPERATIONS_LIST_B = [

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
