use ::{
    derive_more::{self, DebugCustom, Display},
    std::ops::*,
};

macro_rules! primitives {
    ($(
        pub struct $name:ident {
            wrapping $inner:path;
            $(try_from $($try_from:path ),+;)*
            $(deriving $($deriving:path ),+;)*
            $(operator $($trait:ident::$method:ident ),+;)*
        }
    )+) => {$(
        #[derive(
            Clone,
            Copy,
            PartialEq,
            PartialOrd,
            derive_more::Display,
            derive_more::DebugCustom,
            derive_more::From,
            derive_more::Into
            $($(, $deriving)*)*
        )]

        pub struct $name(pub $inner);

        impl Deref for $name {
            type Target = $inner;
            fn deref(&self) -> &$inner {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }

        impl From<bool> for $name {
            fn from(other: bool) -> $name {
                $name(if other { 1u8.into() } else { 0u8.into() })
            }
        }

        impl From<$name> for bool {
            fn from(other: $name) -> bool {
                other.0 != 0u8.into()
            }
        }


        $($(
            impl<T: Into<$name>> $trait<T> for $name {
                type Output = $name;

                fn $method(self, other: T) -> $name {
                    $name(self.0 + other.into().0)
                }
            }
        )*)*

        $($(
            impl From<$try_from> for $name {
                fn from(other: $try_from) -> $name {
                    $name::try_from(other).unwrap()
                }
            }
        )*)*
    )+};

    (@trait_impl $trait:ident @ ($($try_from:path),*)) => {
        $($(
            impl $trait<$try_from> for $name {
                type Output = $name;

                fn $method(self, other: $try_from) -> $name {
                    let other: $name = other.into();
                    $name(self.0.add(other.0))
                }
            }
        )*)*
    };
}

primitives! {
    pub struct Int {
        wrapping i128;
        try_from usize, u8, u16, u32, u64, u128,
                 isize, i8, i16, i32, i64;
        deriving Eq, Ord, Hash;
        operator Add::add, Sub::sub, Mul::mul, Div::div;
    }

    pub struct Offset {
        wrapping isize;
        try_from usize, u8, u16, u32, u64, u128,
                        i8, i16, i32, i64, i128;
        deriving Eq, Ord;
        operator Add::add, Sub::sub, Mul::mul, Div::div;
    }

    pub struct Float {
        wrapping f64;
        try_from f32,
                 i8, i16, i32,
                 u8, u16, u32;
    }
}
