use ::std::ops::*;

macro_rules! primitives {
    ($(
        pub struct $Wrapper:ident {
            wrapping $Inner:path;
            $(try_from $($try_from:path ),+;);*
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

        pub struct $Wrapper(pub $Inner);

        impl Deref for $Wrapper {
            type Target = $Inner;
            fn deref(&self) -> &$Inner {
                &self.0
            }
        }

        impl DerefMut for $Wrapper {
            fn deref_mut(&mut self) -> &mut $Inner {
                &mut self.0
            }
        }

        impl From<bool> for $Wrapper {
            fn from(other: bool) -> $Wrapper {
                $Wrapper(if other { 1u8.into() } else { 0u8.into() })
            }
        }

        impl From<$Wrapper> for bool {
            fn from(other: $Wrapper) -> bool {
                other.0 != 0u8.into()
            }
        }

        $($(
            impl From<$try_from> for $Wrapper {
                fn from(other: $try_from) -> $Wrapper {
                    $Wrapper::try_from(other).unwrap()
                }
            }
        )*)*

        primitives_impl_operators_1!{
            {wrapper=$Wrapper}
            {try_from=$({
                $($try_from);*
            });*}
            names_and_methods=[
                $($(
                    $trait
                    $method
                )*)*
            ]
        }
    )+};
}

macro_rules! primitives_impl_operators_1 {
    {
        {wrapper=$Wrapper:ident}
        {try_from=$({
            $($try_from:path);*
        });*}
        names_and_methods=[$TraitsAndMethods:tt]
    } => {
        primitives_impl_operators_2!{
            {wrapper=$Wrapper}
            {try_from={
                $($($try_from);*)*
            }}
            {names_and_methods=[
                $TraitsAndMethods
            ]}
        }
    }
}

macro_rules! primitives_impl_operators_2 {
    {
        {wrapper=$Wrapper:ident}
        {try_from={
            $($try_from:path);*
        }}
        {names_and_methods=[
            $(
                $trait:ident
                $method:ident
            )*
        ]}
    } => {
        $(
            $(
                impl<T: Into<$Wrapper>> $try_from<T> for $Wrapper {
                    type Output = $Wrapper;

                    fn $method(self, other: T) -> $Wrapper {
                        $Wrapper(self.0 + other.into().0)
                    }
                }
            )*
        )*
    }
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
