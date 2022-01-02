use ::std::ops::*;

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

primitives! {
    pub struct Int(i128) {
        // All values of these types can be exactly represented by an Int.
        // Implements From<each of these types> for Self.
        From { usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, i128 }
        // Some values of these types can be represented exactly by an Int.
        // Other values are out-of-bounds and can not be represented.
        // Implements TryFrom<each of these types> for Self.
        TryFrom { u128 }
        // All values of these types can be approximately represented by an Int.
        // They may experience rounding, but they will not be out-of-bounds.
        ApproximateFrom { }
        // Some values of these types can be approximately represented by an Int.
        // Some may experience rounding, and some will be out-of-bounds.
        TryApproximateFrom { f32, f64 }
        // Derive these traits using std.
        derive { Copy, PartialEq, Eq, PartialOrd, Ord, Hash }
        // Derive these traits using the derive_more crate.
        derive_more { Display, DebugCustom, From, Into }
        // Derive these traits delegating standard unary operators.
        derive_unary_ops {
            Neg::neg,
            Not::not
        }
        // Derive these traits delegating standard binary operators.
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
        rounded_from { Int, usize, u64, u128, isize, i64, i128 }
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
