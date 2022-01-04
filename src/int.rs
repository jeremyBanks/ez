#![allow(unused)]

use {crate::*, ::core::ops::*, ::num_traits::*, ::paste::paste};

// This is Into<T>
trait ToExact<T> {
    fn to_exact(&self) -> T;
}

trait ToApproximate<T> {
    fn to_approximate(&self) -> T;
}

trait TryToExact<T> {
    fn try_to_exact(&self) -> Fallible<T>;
}

// This is similar to num_traits::NumCast
trait TryToApproximate<T> {
    fn try_to_approximate(&self) -> Fallible<T>;
}

macro_rules! def {
    {
        pub struct $Outer:ident($Inner:ident);

        implicit_from $implicit_from:tt;
        implicit_try_from $implicit_try_from:tt;
        explicit_from $explicit_from:tt;
        explicit_try_from $explicit_try_from:tt;
        explicit_parse_from $explicit_parse_from:tt;
        index_as_usize <$T:ident> $index_as_usize:tt;
        derive_std $derive_std:tt;
        derive_more $derive_more:tt;
        derive_unary_operators $derive_unary_operators:tt;
        derive_binary_operators $derive_binary_operators:tt;
    } => { paste! { def! { @with_pasted_idents {
        Outer { [<$Outer>] };
        outer { [<$Outer:snake>] };
        ToOuter { [<To $Outer>] };
        to_outer { [<to_ $Outer:snake>] };
        Inner { [<$Inner>] };
        inner { [<$Inner:snake>] };
        implicit_from $implicit_from;
        implicit_try_from $implicit_try_from;
        explicit_from $explicit_from;
        explicit_try_from $explicit_try_from;
        explicit_parse_from $explicit_parse_from;
        index_as_usize <$T> $index_as_usize;
        derive_std $derive_std;
        derive_more $derive_more;
        derive_unary_operators $derive_unary_operators;
        derive_binary_operators $derive_binary_operators;
    } } } };

    { @with_pasted_idents {
        Outer { $Outer:ident };
        outer { $outer:ident };
        ToOuter { $ToOuter:ident };
        to_outer { $to_outer:ident };
        Inner { $Inner:ident };
        inner { $inner:ident };
        implicit_from $implicit_from:tt;
        implicit_try_from $implicit_try_from:tt;
        explicit_from $explicit_from:tt;
        explicit_try_from $explicit_try_from:tt;
        explicit_parse_from $explicit_parse_from:tt;
        index_as_usize <$T:ident> $index_as_usize:tt;
        derive_std $derive_std:tt;
        derive_more $derive_more:tt;
        derive_unary_operators $derive_unary_operators:tt;
        derive_binary_operators $derive_binary_operators:tt;
    } } => {
        #[derive(Copy, Clone, Debug)]
        pub struct $Outer($Inner);

        pub trait $ToOuter {
            fn $to_outer(&self) -> $Outer;
        }

        impl $ToOuter for $Outer {
            fn $to_outer(&self) -> $Outer {
                *self
            }
        }

        def! { @derive_from {
            Outer { $Outer };
            from $implicit_from;
        } }

        def! { @derive_from {
            Outer { $Outer };
            from $explicit_from;
        } }

        def! { @derive_try_from {
            Outer { $Outer };
            try_from $implicit_try_from;
        } }

        def! { @derive_try_from {
            Outer { $Outer };
            try_from $explicit_try_from;
        } }

        def! { @derive_operators_for_self {
            Outer { $Outer };
            derive_unary_operators $derive_unary_operators;
            derive_binary_operators $derive_binary_operators;
        } }

        def! { @derive_operators_for_implicit {
            Outer { $Outer };
            implicit_from $implicit_from;
            implicit_try_from $implicit_try_from;
            derive_binary_operators $derive_binary_operators;
        } }
    };

    { @derive_from {
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

    { @derive_try_from {
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

    { @derive_operators_for_self $($_tt:tt)* } => {};

    { @derive_operators_for_implicit $($_tt:tt)* } => {};
}

def! {
    pub struct Int(i128);
    implicit_from {
        bool, u8, u16, u32, u64, i8, i16, i32, i64, i128,
    };
    implicit_try_from { u128, usize, isize };
    explicit_from {};
    explicit_try_from { f32, f64 };
    explicit_parse_from { &str, String, };
    index_as_usize<T> { &[T], Vec<T>, };
    derive_std { Copy, PartialEq, Eq, PartialOrd, Ord, Hash, };
    derive_more { Display, DebugCustom, From, Into, };
    derive_unary_operators {
        ::core::ops::Neg::neg,
        ::core::ops::Not::not,
    };
    derive_binary_operators {
        ::core::ops::Add.add,
        ::core::ops::Sub.sub,
        ::core::ops::Mul.mul,
        ::core::ops::Div.div,
        ::core::ops::Rem.rem,
        ::core::ops::BitAnd.bitand,
        ::core::ops::BitOr.bitor,
        ::core::ops::BitXor.bitxor,
        ::core::ops::Shl.shl,
        ::core::ops::Shr.shr,
        ::num_traits::Pow.pow,
    };
}

macro_rules! primitives {
    ($(
        pub struct $Wrapper:ident($Inner:path) {
            $(from_any { $($from_any:path ),* });*
            $(from_some { $($from_some:path ),* });*
            $(rounded_from { $($rounded_from:path ),* });*
            $(rounded_from_some { $($rounded_from_some:path ),* });*
            $(derive { $($derive:path ),* });*
            $(derive_more { $($derive_more:ident ),* });*
            $(derive_unary_ops { $($unary_trait:ident::$unary_method:ident ),* });*
            $(derive_binary_ops { $($binary_trait:ident::$binary_method:ident ),* });*
        }
    )+) => {$(
        #[derive(
            Default,
            Clone
            $($(, $derive)*)*
            $($(, derive_more::$derive_more)*)*
        )]

        pub struct $Wrapper(pub $Inner);

        impl From<bool> for $Wrapper {
            fn from(other: bool) -> $Wrapper {
                $Wrapper(if other { 1u8.into() } else { 0u8.into() })
            }
        }

        impl From<$Wrapper> for bool {
            fn from(other: $Wrapper) -> bool {
                other.0
            }
        }

        primitives_impl_operators_1!{
            wrapper = $Wrapper;
            try_from = $(
                $($from_any),*
            ),*;
            traits_and_methods = [
                $($(
                    [$binary_trait $binary_method]
                )*)*
            ]
        }
    )+};
}

macro_rules! primitives_impl_operators_1 {
    {
        wrapper = $Wrapper:ident;
        try_from = $($try_from:path),*;
        traits_and_methods = $TraitsAndMethods:tt
    } => {
        primitives_impl_operators_2!{
            wrapper = $Wrapper;
            $(pairs = {
                try_from = $try_from;
                traits_and_methods = $TraitsAndMethods
            }),*
        }
    }
}

macro_rules! primitives_impl_operators_2 {
    {
        wrapper = $Wrapper:ident;
        $(pairs = {
            try_from = $try_from:path;
            traits_and_methods = [$([$trait:ident $method:ident])*]
        }),*
    } => {
        $(
            $(
                impl ::std::ops::$trait<$try_from> for $Wrapper {
                    type Output = $Wrapper;

                    fn $method(self, other: $try_from) -> $Wrapper {
                        let other: $Wrapper = other.into();
                        $Wrapper(self.0.$method(other.0))
                    }
                }

                impl ::std::ops::$trait<$Wrapper> for $try_from {
                    type Output = $Wrapper;

                    fn $method(self, other: $Wrapper) -> $Wrapper {
                        let self_: $Wrapper = self.into();
                        $Wrapper(self_.0.$method(other.0))
                    }
                }
            )*
        )*
    }
}
