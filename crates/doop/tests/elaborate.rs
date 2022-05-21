use {
    doop::*,
    std::{
        error::Error,
        fmt::{Debug, Display},
    },
};

type IntInner = i128;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int(IntInner);

doop! {
    let IntFrom = [ bool, u8, u16, u32, u64, i8, i16, i32, i64, i128, ];
    let IntTryFrom = [ u128, usize, isize, ];
    let IntTryFromApproximate = [ f32, f64, ];
    let IntInto = [ i128, ];
    let IntTryInto = [ u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize, ];
    let IntIntoApproximate = [ f32, f64, ];

    type Reportable = 'static + Error + Display + Send + Sync;

    for Other in IntFrom {
        impl From<Other> for Int {
            fn from(value: Other) -> Int {
                Int(value.try_into().unwrap())
            }
        }
    }

    for Other in IntInto {
        impl From<Int> for Other {
            fn from(value: Int) -> Other {
                value.0.try_into().unwrap()
            }
        }
    }

    for Dx in [Debug, Display] {
        impl Dx for Int {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }

    let BinaryOpsGeneric = [
        (::core::ops::Add, add),
        (::core::ops::Sub, sub),
        (::core::ops::Mul, mul),
        (::core::ops::Div, div),
        (::core::ops::Rem, rem),
        (::core::ops::BitAnd, bitand),
        (::core::ops::BitOr, bitor),
        (::core::ops::BitXor, bitxor),
    ];

    let BinaryOpsU32 = [
        (::core::ops::Shl, shl),
        (::core::ops::Shr, shr),
        (::num_traits::Pow, pow),
    ];

    for (Op, method) in BinaryOpsGeneric {
        impl Op<Int> for Int {
            type Output = Int;

            fn method(self, rhs: Int) -> Self::Output {
                let left: IntInner = self.0;
                let right: IntInner = Int::from(rhs).0;
                let result: IntInner = Op::method(left, right);
                Int(result)
            }
        }

        impl Op<Option<Int>> for Int {
            type Output = Option<Int>;

            fn method(self, rhs: Option<Int>) -> Self::Output {
                let left: IntInner = self.0;
                let right: IntInner = Int::from(rhs?).0;
                let result: IntInner = Op::method(left, right);
                Some(Int(result))
            }
        }

        impl Op<Int> for Option<Int> {
            type Output = Option<Int>;

            fn method(self, rhs: Int) -> Self::Output {
                let left: IntInner = self?.0;
                let right: IntInner = Int::from(rhs).0;
                let result: IntInner = Op::method(left, right);
                Some(Int(result))
            }
        }

        impl<E: Reportable> Op<Result<Int, E>> for Int {
            type Output = Result<Int, eyre::Report>;

            fn method(self, rhs: Result<Int, E>) -> Self::Output {
                let left: IntInner = self.0;
                let right: IntInner = Int::from(rhs?).0;
                let result: IntInner = Op::method(left, right);
                Ok(Int(result))
            }
        }

        impl<E: Reportable> Op<Int> for Result<Int, E> {
            type Output = Result<Int, eyre::Report>;

            fn method(self, rhs: Int) -> Self::Output {
                let left: IntInner = self?.0;
                let right: IntInner = Int::from(rhs).0;
                let result: IntInner = Op::method(left, right);
                Ok(Int(result))
            }
        }
    }

    for (Op, method) in BinaryOpsGeneric
    for Other in IntFrom
    {
        impl Op<Other> for Int {
            type Output = Int;

            fn method(self, rhs: Other) -> Self::Output {
                let left: IntInner = self.0;
                let right: IntInner = Int::from(rhs).0;
                let result: IntInner = Op::method(left, right);
                Int(result)
            }
        }

        impl Op<Int> for Other {
            type Output = Int;

            fn method(self, rhs: Int) -> Self::Output {
                let left: IntInner = Int::from(self).0;
                let right: IntInner = rhs.0;
                let result: IntInner = Op::method(left, right);
                Int(result)
            }
        }
    }
}

#[test]
fn test() {
    let x = Int::from(2);

    let y = x + x * x - x / x * 3 + 1 - 2 + 3;

    assert_eq!(Int::from(5), y);
}
