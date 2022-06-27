

// pub fn int(_: impl std::any::Any) -> Int {
//     todo!()
// }

// mod convert;
// use self::convert::*;

// impl TryToInt for &str {
//     #[throws]
//     fn try_to_int(&self) -> Int {
//         Int(self.parse()?)
//     }
// }

/*

// XXX: where possible maybe implement for Result and/or Option?

doop! {
    let TrueFromTypes = [u8, u16, u32, u64, usize, i8, i16, i32, i64, i128,
isize];     let PseudoFromTypes = [usize, isize];
    let BinaryOps = [ (Add, add), (Sub, sub), (Mul, mul), (Div, div) ];
    let FromTypes = TrueFromTypes + PseudoFromTypes + [u128];

    for Type in FromTypes + [u128]
    for (Trait, Method) in BinaryOps
    {
        impl ToInt for Type {
            fn to_int(&self) -> Int {
                todo!()
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
}

doop! {
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

   impl From { bool, u8, u16, u32, u64, i8, i16, i32, i64, i128, }
   impl TryFrom { u128, usize, isize, }
   impl TryFromApproximate { f32, f64, }
   impl Into { i128, }
   impl TryInto { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize, }
   impl IntoApproximate { f32, f64, }

   impl<T> Index<usize> for { &[T], Vec<T> }
   // Do we just want to coerce to usize, or do we want more magic (probably
not), such as:    // When slicing a string, we should round each index
backwards to the nearest valid character break    // instead of panicking.
   // When indexing with negative values, they should be measured from
.len().

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
doop! {
    let UnsignedTypes = [u8, u16, u32];
    let SignedTypes = UnsignedTypes + [
        impl ::num_traits::Num,
        impl ::num_traits::NumCast,
        2,
        (4 + 4),
        core::i128
    ];
    let Something = SignedTypes - UnsignedTypes + [2];

    let SignedTypes = [i8, i16, i32] + [{impl ::num_traits::Num}, {impl
SomethingElse}];     let BinaryOps = [+, -, /];
    let Types = *SignedTypes + UnsignedTypes;

    // Each piece of code to be duplicated is indicated with a
// `for`-loop-style     // block, optionally with multiple `for` statements for
// nested repetitions.
    for Type in Types
    for B in BinaryOps {
        let n: Type = 1;
        dbg!(n B n);
    }

    // `for` can refer to aliases and/or bracketed token trees, just like
    // `let`.
    for Type in *Types + [u128, i128]
    for U in [!, -] {
        let n: Type = 2.try_into().unwrap();
        dbg!(U n);
    }
}
 */
