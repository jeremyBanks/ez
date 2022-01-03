use ::core::ops::*;
use ::paste::paste;
use ::eyre::{bail as throw, eyre as error, ensure as assert, WrapErr as _};

pub type Fallible<T> = Result<T, ::eyre::Report>;

// This is just Into?
trait ToExact<T> {
    fn to_exact(&self) -> T;
}

trait ToApproximate<T> {
    fn to_approximate(&self) -> T;
}

trait TryToExact<T> {
    fn try_to_exact(&self) -> Fallible<T>;
}

trait TryToApproximate<T> {
    fn try_to_approximate(&self) -> Fallible<T>;
}

// Gives us int(true), int(10usize)
// Gives us int(3).unwrap_u8() (panicking)
// Gives us int(3).u8() (result, like TryInto but shorter)
// Gives us u8::try_from(int(3)).unwrap()

// Should we implicitly cast *TO* usize when operating with it?
// Would that give us 0+some int as a valid index? Probably would confuse inference

def! {
    pub struct Int(i128);

    implicit_from { bool, u8, u16, u32, u64, usize, i8, i16, i32, i64, i128, isize };

    implicit_try_from { u128 };

    explicit_from {};

    explicit_try_from { f32, f64, Float };

    explicit_parse_from { &str, String };

    derive_std { Copy, PartialEq, Eq, PartialOrd, Ord, Hash };

    derive_more { Display, DebugCustom, From, Into };

    derive_unary_operations { Neg::neg, Not::not };

    derive_binary_operations {
        Add::add,
        Sub::sub,
        Mul::mul,
        Div::div,
        Rem::rem,
        BitAnd::bitand,
        BitOr::bitor,
        BitXor::bitxor,
        Shl::shl,
        Shr::shr,
    };
}

macro_rules! def {
    {
        pub struct $Outer:ident($Inner:ident);
    } => {
        def! { @desugared { paste! {
            Outer = [<$Outer>],
            outer = [<$Outer::snake>],
            ToOuter = [<To $Outer>]
            to_outer = [<to_ $Outer::snake>],
            Inner = [<$Inner>],
            inner = [<$inner::snake>],
        } } }
    },

    { @desugared { paste! {
        Outer = $Outer:ident,
        outer = $outer:ident,
        ToOuter = $ToOuter:ident,
        to_outer = $to_outer:ident,
        Inner = $Inner:ident;
        inner = $inner:ident;
    } } } => {
        pub struct $Outer($Inner);

        pub trait $ToOuter {
            fn $to_outer(&self) -> $Outer;
        }

        impl $ToOuter for $Outer {
            fn $to_outer(&self) -> $Outer {
                *self
            }
        }
    },
}

pub trait IntParsable {
    fn to_int(&self) -> Int;
}

pub fn int(n: impl IntParsable) -> Int {
    n.to_int()
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
