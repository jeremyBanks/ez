#![warn(unused_crate_dependencies)]

use {
    ez::{throws, try_throws, __::repeat},
    paste::paste,
};

#[derive(
    ::core::clone::Clone,
    ::core::cmp::Eq,
    ::core::cmp::Ord,
    ::core::cmp::PartialEq,
    ::core::cmp::PartialOrd,
    ::core::default::Default,
    ::core::hash::Hash,
    ::core::marker::Copy,
    ::derive_more::AsMut,
    ::derive_more::AsRef,
    ::derive_more::DebugCustom,
    ::derive_more::Deref,
    ::derive_more::DerefMut,
    ::derive_more::Display,
    ::derive_more::FromStr,
    ::num_derive::FromPrimitive,
    ::num_derive::Num,
    ::num_derive::NumCast,
    ::num_derive::NumOps,
    ::num_derive::One,
    ::num_derive::ToPrimitive,
    ::num_derive::Zero,
    ::serde::Deserialize,
    ::serde::Serialize,
)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Int(i128);

mod convert;
use self::convert::*;

impl TryToInt for &str {
    #[throws]
    fn try_to_int(&self) -> Int {
        Int(self.parse()?)
    }
}

repeat! {
    // where Type: IntoInt
    for Type in [u8, u16, u32, u64, usize, i8, i16, i32, i64, i128, isize] {
        impl ToInt for Type {
            fn to_int(&self) -> Int {
                Int((*self).try_into().unwrap())
            }
        }

        impl std::ops::Add<Int> for Type {
            type Output = Int;

            fn add(self, rhs: Int) -> Self::Output {
                self.to_int() + rhs
            }
        }

        impl std::ops::Add<Type> for Int {
            type Output = Int;

            fn add(self, rhs: Type) -> Self::Output {
                self + rhs.to_int()
            }
        }
    }

    // where Type: TryIntoInt + !IntoInt
    for Type in [u128] {
        impl ToInt for Type {
            fn to_int(&self) -> Int {
                Int((*self).try_into().unwrap())
            }
        }

        impl std::ops::Add<Int> for Type {
            type Output = Int;

            fn add(self, rhs: Int) -> Self::Output {
                self.try_to_int().unwrap() + rhs
            }
        }

        impl std::ops::Add<Type> for Int {
            type Output = Int;

            fn add(self, rhs: Type) -> Self::Output {
                self + rhs.try_to_int().unwrap()
            }
        }
    }
}

#[try_throws]
pub fn int(i: impl TryToIntApproximate) -> Int {
    i.try_to_int_approximate()?
}

/*
   impl From { bool, u8, u16, u32, u64, i8, i16, i32, i64, i128, }
   impl TryFrom { u128, usize, isize, }
   impl TryFromApproximate { f32, f64, }
   impl Into { i128, }
   impl TryInto { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize, }
   impl IntoApproximate { f32, f64, }

   impl<T> Index<usize> for { &[T], Vec<T> }
   // Do we just want to coerce to usize, or do we want more magic (probably not), such as:
   // When slicing a string, we should round each index backwards to the nearest valid character break
   // instead of panicking.
   // When indexing with negative values, they should be measured from .len().

   impl BinaryOperator<Self, Output=Self> {
       [::core::ops::Add]::add,
       [::core::ops::Sub]::sub,
       [::core::ops::Mul]::mul,
       [::core::ops::Div]::div,
       [::core::ops::Rem]::rem,
       [::core::ops::BitAnd]::bitand,
       [::core::ops::BitOr]::bitor,
       [::core::ops::BitXor]::bitxor,
   }

   impl BinaryOperator<u32, Output=Self> {
       [::core::ops::Shl]::shl,
       [::core::ops::Shr]::shr,
       [::num_traits::Pow]::pow,
   }

   // We should have #[inherent]-style shims for as much as it makes sense.
*/
