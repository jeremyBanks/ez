use ::std::ops::*;

macro_rules! primitives {
    ($(
        pub struct $Wrapper:ident($Inner:path) {
            $(from_any { $($from_any:path ),+ });*
            $(from_some { $($from_some:path ),+ });*
            $(rounded_from { $($rounded_from:path ),+ });*
            $(rounded_from_some { $($rounded_from_some:path ),+ });*
            $(derive { $($derive:path ),+ });*
            $(derive_more { $($derive_more:ident ),+ });*
            $(derive_unary_ops { $($unary_trait:ident::$unary_method:ident ),+ });*
            $(derive_binary_ops { $($binary_trait:ident::$binary_method:ident ),+ });*
        }
    )+) => {$(
        #[derive(
            Clone
            $($(, $derive)*)*
            $($(, derive_more::$derive_more)*)*
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
            impl From<$from_any> for $Wrapper {
                fn from(other: $from_any) -> $Wrapper {
                    $Wrapper::try_from(other).unwrap()
                }
            }
        )*)*

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

primitives! {
    pub struct Int(i128) {
        from_any { usize, u8, u16, u32, u64, isize, i8, i16, i32, i64 }
        from_some { u128 }
        rounded_from_some { f32, f64 }
        derive { Copy, PartialEq, Eq, PartialOrd, Ord, Hash }
        derive_more { Display, DebugCustom, From, Into }
        derive_unary_ops {
            Neg::neg,
            Not::not
        }
        derive_binary_ops {
            Add::add,
            Sub::sub,
            Mul::mul,
            Div::div,
            BitAnd::bitand,
            BitOr::bitor,
            BitXor::bitxor,
            Rem::rem,
            Shl::shl,
            Shr::shr
        }
    }

    pub struct Float(f64) {
        from_any { f32 }
        from_some { u8, u16, u32, i8, i16, i32 }
        rounded_from { usize, u64, u128, isize, i64, i128 }
        derive { Copy }
        derive_more { Display, DebugCustom, From, Into }
        derive_unary_ops {
            Not::not
        }
        derive_binary_ops {
            Add::add,
            Sub::sub,
            Mul::mul,
            Div::div
        }
    }
}

pub trait IntParsable {
    fn to_int(&self) -> Int;
}

pub fn int(n: impl IntParsable) -> Int {
    n.to_int()
}
