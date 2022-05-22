#[allow(non_upper_case_globals)]
// If we want to match normal syntax for the sake of rustfmt,
// we can replace
//    doop! {
// with
//    static doop: ! = {
#[doop::pseudo]
static doop: ! = {
    let BOOL: TokenStream = BOOL;

    // vec![] is a way to add a macro context with brackets
    

    let int_from: [TokenStream] = vec![BOOL, u8, u16, u32, u64, i8, i16, i32, i64, i128];
    let int_try_from: [TokenStream] = vec![u128, usize, isize];
    let int_try_from_approximate: [TokenStream] = [f32, f64] + [] - [];
    let int_maybe_from: [TokenStream] = int_from + int_try_from + int_try_from_approximate;

    // static is for no interpolation?
    // If that's an option. Probably shouldn't be.
    static int_maybe_from: [TokenStream] = int_from + int_try_from + int_try_from_approximate;

    // do we want to allow interpolation of array values to create array results?
    // maybe. if the expected result type is multiple values? yeah.
    // so what do we do with let? infer?

    let int_into: [TokenStream] = [i128];
    let int_try_into: [TokenStream] =
        [BOOL, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize];
    let int_into_approximate: [TokenStream] = [f32, f64];
    let int_maybe_into: [TokenStream] = int_into + int_try_into + int_into_approximate;


    // `let`, `type`, and `const` are equivalent (interpolated),
    // they have different suggested capitalizations and unescaped grammar.
    // static too, 'cause why not?
    let static_lifetime: TokenStream = ungrammatical!('static);
    type static_lifetime: TokenStream = ungrammatical!(Lifetime static);
    const static_lifetime: TokenStream = ungrammatical!('static);
    type static_lifetime: TokenStream = Any + 'static;

    'static: {
        let x = 2;
    }

    let reportable: TokenStream = static_lifetime + join!(E rr or) + Display + Send + Sync;
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
