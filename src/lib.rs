#![allow(unused)]

use {
    ::eyre::{bail as throw, ensure, eyre as error, Report, Result as Fallible, WrapErr},
    num_derive::{FromPrimitive, ToPrimitive},
};

macro_rules! implicint {
    {
        $(#$attributes:tt)*
        pub struct $Outer:ident($Inner:ident);
        impl From $From:tt
        impl TryFrom $TryFrom:tt
        impl TryFromApproximate $explicit_from_approximate:tt
        impl Into $Into:tt
        impl TryInto $TryInto:tt
        impl IntoApproximate $TryIntoApproximate:tt
        impl<$T:ident> Index<usize> for $index_as_usize:tt
        impl BinaryOperator<Self, Output=Self> $delegate_binary_traits:tt
        impl BinaryOperator<u32, Output=Self> $delegate_binary_u32_traits:tt
    } => {
        ::paste::paste! {
            implicint! {
                @desugared {
                    attributes { $(#$attributes)* };
                    Outer { [<$Outer>] };
                    outer { [<$Outer:snake>] };
                    ToOuter { [<To $Outer>] };
                    to_outer { [<to_ $Outer:snake>] };
                    Inner { [<$Inner>] };
                    inner { [<$Inner:snake>] };
                    from $From;
                    try_from $TryFrom;
                    explicit_from_approximate $explicit_from_approximate;
                    index_as_usize<$T> $index_as_usize;
                    delegate_binary_traits $delegate_binary_traits;
                    delegate_binary_u32_traits $delegate_binary_u32_traits;
                }
            }
        }
    };

    {
        @desugared {
            attributes { $(#$attributes:tt)* };
            Outer { $Outer:ident };
            outer { $outer:ident };
            ToOuter { $ToOuter:ident };
            to_outer { $to_outer:ident };
            Inner { $Inner:ident };
            inner { $inner:ident };
            from $From:tt;
            try_from $TryFrom:tt;
            explicit_from_approximate $explicit_from_approximate:tt;
            index_as_usize<$T:ident> $index_as_usize:tt;
            delegate_binary_traits $delegate_binary_traits:tt;
            delegate_binary_u32_traits $delegate_binary_u32_traits:tt;
        }
    } => {
        $(#$attributes)*
        pub struct $Outer($Inner);
        pub fn $outer(x: impl ::std::fmt::Debug) -> $Outer {
            todo!();
        }

        implicint! {
            @delegate_from {
                Outer { $Outer };
                from $From;
            }
        }

        implicint! {
            @delegate_try_from {
                Outer { $Outer };
                try_from $TryFrom;
            }
        }
    };

    { @delegate_from {
        Outer { $Outer:ident };
        from { $($Type:ty),* $(,)? };
    } } => {
        $(
            impl From<$Type> for $Outer {
                fn from(other: $Type) -> $Outer {
                    $Outer(other.into())
                }
            }
        )*
    };

    { @delegate_try_from {
        Outer { $Outer:ident };
        try_from { $($Type:ty),* $(,)? };
    } } => {
        $(
            impl TryFrom<$Type> for $Outer {
                type Error = Report;
                fn try_from(other: $Type) -> Fallible<$Outer> {
                    Ok($Outer(other.try_into()?))
                }
            }
        )+
    };

    { @delegate_unary_traits {
        Outer { $Outer:ident };
        impl unary operators { $([$($UnaryTrait:tt)+]::$unary_method:ident),* $(,)? };
    } } => {
        $(
            #[::inherent::inherent]
            impl $($UnaryTrait)+ for $Outer {
                type Output = $Outer;
                fn $unary_method(self) -> $Outer {
                    $Outer(self.0.$unary_method())
                }
            }
        )*
    };

    { @delegate_binary_traits {
        Outer { $Outer:ident };
        Other { $Other:ident };
        delegate_binary_traits { $([$($BinaryTrait:tt)+]::$binary_method:ident),* $(,)? };
    } } => {
        $(
            impl $($BinaryTrait)*::<$Other> for $Outer {
                type Output = $Outer;
                fn $binary_method(self, other: $Other) -> $Outer {
                    let other = $Outer::from(other);
                    $Outer(self.0.$binary_method(other.0))
                }
            }
        )*
    };

    { @delegate_binary_u32_traits {
        Outer { $Outer:ident };
        Other { $Other:ident };
        delegate_binary_u32_traits { $([$($BinaryTrait:tt)+]::$binary_method:ident),* $(,)? };
    } } => {
        $(
            impl $($BinaryTrait)*::<$Other> for $Outer {
                type Output = $Outer;
                fn $binary_method(self, other: $Other) -> $Outer {
                    // let other = u32::try_from(other).unwrap();
                    // $Outer(self.0.$binary_method(other))
                    todo!()
                }
            }
        )*
    };
}

implicint! {
    #[derive(
        ::core::clone::Clone,
        ::core::cmp::Eq,
        ::core::cmp::Ord,
        ::core::cmp::PartialEq,
        ::core::cmp::PartialOrd,
        ::core::hash::Hash,
        ::core::marker::Copy,
        ::derive_more::DebugCustom,
        ::derive_more::Display,
        ::derive_more::Deref,
        ::derive_more::DerefMut,
        ::derive_more::FromStr,
        ::derive_more::AsRef,
        ::derive_more::AsMut,
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
}

#[test]
fn test_implicint() -> Fallible<()> {
    use num_traits::Zero;

    let a: i8 = 101;
    let b: u32 = 1001;
    let c: i64 = 2002;
    let d: u128 = 3003;

    let quad = a + int(2) + b + c + d;

    Ok(())
}
