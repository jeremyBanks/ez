#[doop::pseudo]
const _: x = {
    let BOOL: TokenStream = BOOL;

    let int_from: [TokenStream] = [BOOL, u8, u16, u32, u64, i8, i16, i32, i64, i128];
    let int_try_from: [TokenStream] = [u128, usize, isize];
    let int_try_from_approximate: [TokenStream] = [f32, f64] + [] - [];
    let int_maybe_from: [TokenStream] = int_from + int_try_from + int_try_from_approximate;

    // static is for no interpolation
    static int_maybe_from: [TokenStream] = int_from + int_try_from + int_try_from_approximate;

    // do we want to allow interpolation of array values to create array results?
    // maybe. if the expected result type is multiple values? yeah.
    // so what do we do with let? infer?

    let int_into: [TokenStream] = [i128];
    let int_try_into: [TokenStream] =
        [BOOL, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize];
    let int_into_approximate: [TokenStream] = [f32, f64];
    let int_maybe_into: [TokenStream] = int_into + int_try_into + int_into_approximate;

    type Lifetime = punctuation!["'"];

    // `let`, `type`, and `const` are equivalent (interpolated),
    // they have different suggested capitalizations and unescaped grammar.
    let static_lifetime: TokenStream = escape! {'static};
    type static_lifetime: TokenStream = escape! {Lifetime static};
    const static_lifetime: TokenStream = escape! {'static};
    type static_lifetime: TokenStream = Any + 'static;

    let reportable: TokenStream = static_lifetime + Error + Display + Send + Sync;
    let reportable: TokenStream = static_lifetime + Error + Display + Send + Sync;

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
