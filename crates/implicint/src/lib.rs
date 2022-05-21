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
    thiserror::Error,
    paste::paste,
};

// one thing at a fucking time, jesus christ.
// first, basic ops, checked and panicking(!).
// then add coercion from strictly compatible types.
// then add .into() and .from() for all integer types, panicking or throwing as appropriate.
// then add int() converting from approximately maybe compatible types.



#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Default, Hash, Copy)]
pub struct Int<M> {
    value: i128,
}

#[derive(Error, Debug)]
pub enum IntError {
    #[error(transparent)]
    AddOverflow(IntAddOverflowError),
    #[error(transparent)]
    SubOverflow(Int, Int),
    #[error(transparent)]
    MulOverflow(Int, Int),
    #[error(transparent)]
    DivisionByZero(Int),
}

#[derive(Error, Debug)]
#[error("result overflowed when adding: `{.lhs} + {.rhs}`")]
pub struct IntAddOverflowError {
    pub lhs: Int,
    pub rhs: Int,
}

#[derive(Error, Debug)]
#[error("result overflowed when subtracting: `{.lhs} - {.rhs}`")]
pub struct IntSubOverflowError {
    pub lhs: Int,
    pub rhs: Int,
}

#[derive(Error, Debug)]
#[error("result overflowed when multiplying: `{.lhs} * {.rhs}`")]
pub struct IntMulOverflowError {
    pub lhs: Int,
    pub rhs: Int,
}

#[derive(Error, Debug)]
#[error("divided by zero: `{.lhs} / 0`")]
pub struct IntDivisionByZeroError {
    pub lhs: Int,
}


impl Int {
    pub const MAX: Self = Self { value: i128::MAX };
    pub const MIN: Self = Self { value: i128::MIN };
}

pub(crate) trait IntImpl: Into<Int> {
    pub fn try_add(self, rhs: Int) -> Result<Int, IntError> {
        let lhs = self.into();
        let rhs = rhs.into();
        let result = lhs.value.checked_add(rhs.value);
        if let Some(result) = result {
            Ok(result.into())
        } else {
            Err(IntError::AddOverflow(lhs, rhs))
        }
    }
}
impl IntImpl for Int {}

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
