use crate::{
    Fallible,
    Report,
};

macro_rules! jeb {
    {
        $(#$attributes:tt)*
        pub struct $Outer:ident($Inner:ident);
        impl From $From:tt
        impl TryFrom $TryFrom:tt
        impl FromApproximate $explicit_from_approximate:tt
        impl Into $Into:tt
        impl TryInto $TryInto:tt
        impl<$T:ident> Index<Self> for $index_as_usize:tt
        impl fn() $delegate_nullary_traits:tt
        impl fn(self) $delegate_unary_traits:tt
        impl fn(self, Self) $delegate_binary_traits:tt
        impl fn(self, u32::try_from(Self)) $delegate_binary_u32_traits:tt
    } => {
        ::paste::paste! {
            jeb! {
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
                    delegate_unary_traits $delegate_unary_traits;
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
            delegate_unary_traits $delegate_unary_traits:tt;
            delegate_binary_traits $delegate_binary_traits:tt;
            delegate_binary_u32_traits $delegate_binary_u32_traits:tt;
        }
    } => {
        $(#$attributes)*
        pub struct $Outer($Inner);

        impl ::core::str::FromStr for $Outer {
            type Err = Report;
            fn from_str(s: &str) -> Fallible<$Outer> {
                Ok($Outer($Inner::from_str(s)?))
            }
        }

        jeb! {
            @delegate_from {
                Outer { $Outer };
                from $From;
            }
        }

        jeb! {
            @delegate_try_from {
                Outer { $Outer };
                try_from $TryFrom;
            }
        }

        jeb! {
            @delegate_unary_traits {
                Outer { $Outer };
                impl fn(self) $delegate_unary_traits;
            }
        }

        jeb! {
            @delegate_binary_traits {
                Outer { $Outer };
                Other { $Outer };
                delegate_binary_traits $delegate_binary_traits;
            }
        }

        jeb! {
            @delegate_binary_u32_traits {
                Outer { $Outer };
                Other { $Outer };
                delegate_binary_u32_traits $delegate_binary_u32_traits;
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
        impl fn(self) { $([$($UnaryTrait:tt)+]::$unary_method:ident),* $(,)? };
    } } => {
        $(
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
                    let other = u32::try_from(other).unwrap();
                    $Outer(self.0.$binary_method(other))
                }
            }
        )*
    };
}

jeb! {
    #[derive(
        Copy,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::derive_more::Display,
        ::derive_more::DebugCustom,
    )]
    #[repr(transparent)]
    #[serde(transparent)]
    pub struct Int(i128);
    impl From { bool, u8, u16, u32, u64, i8, i16, i32, i64, i128, }
    impl TryFrom { u128, usize, isize, }
    impl FromApproximate { f32, f64, }
    impl Into { i128, }
    impl TryInto { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize, }
    impl<T> Index<Self> for { &[T], Vec<T>, }
    impl fn() {
        [::num::traits::Zero]::zero,
        [::num::traits::One]::one,
    }
    impl fn(self) {
        [::core::ops::Neg]::neg,
        [::core::ops::Not]::not,
    }
    impl fn(self, Self) {
        [::core::ops::Add]::add,
        [::core::ops::Sub]::sub,
        [::core::ops::Mul]::mul,
        [::core::ops::Div]::div,
        [::core::ops::Rem]::rem,
        [::core::ops::BitAnd]::bitand,
        [::core::ops::BitOr]::bitor,
        [::core::ops::BitXor]::bitxor,
    }
    impl fn(self, u32::try_from(Self)) {
        [::core::ops::Shl]::shl,
        [::core::ops::Shr]::shr,
        [::num::traits::Pow]::pow,
    }
}
