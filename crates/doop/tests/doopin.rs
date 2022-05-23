#[doop::X]
static X: ! = {
    let BOOL: TokenStream = BOOL;

    let int_from: [TokenStream] = [BOOL, u8, u16, u32, u64, i8, i16, i32, i64, i128];
    let int_try_from: [TokenStream] = [u128, usize, isize];
    let int_try_from_approximate: [TokenStream] = [f32, f64] + [] - [];
    let int_maybe_from: [TokenStream] = int_from + int_try_from + int_try_from_approximate;

    static int_maybe_from: [TokenStream] = int_from + int_try_from + int_try_from_approximate;

    let int_into: [TokenStream] = [i128];
    let int_try_into: [TokenStream] =
        [BOOL, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize];
    let int_into_approximate: [TokenStream] = [f32, f64];
    let int_maybe_into: [TokenStream] = int_into + int_try_into + int_into_approximate;

    // `let`, `type`, and `const` are equivalent (interpolated),
    // they have different suggested capitalizations and unescaped grammar.
    // static too, 'cause why not? Use what makes the most sense for your code.
    let static_lifetime: TokenStream = X!('static);
    type static_lifetime: TokenStream = X!(Lifetime static);
    type static_lifetime: TokenStream = Any + 'static;

    let static_lifetime = X!('static ;);

    let reportable: TokenStream = static_lifetime + join!(E rr or) + Display + Send + Sync;
    let reportable: TokenStream = static_lifetime + Error + Display + Send + Sync;

    // this is only accepted at the top level, and has the same effect as wrapping
    // the next item in parenthesis. item I think defined as being until after the
    // next semicolon or braced group. it's like to emit stuff without adding
    // another level of indentation.
    #[X]
    let x = 2;

    #[X(for X in [1, 2, 3])]
    let x = X;

    X! {x};

    {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct Int(i128);

        2 + 2;
    }

    let output: [TokenStream] = for SomeInt in int_from {
        impl From<SomeInt> for Int {
            fn from(value: SomeInt) -> Int {
                Int(value.try_into().unwrap())
            }
        }
    };

    {
        output
    }
};
