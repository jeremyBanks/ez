#[doop::from]
static DOOP: ! = {
    // defaults to [TokenStream]
    let my_types: [TokenStream] = [T];
    let my_types: _ = my_type;
    let my_types = my_type;
    // can be set to TokenStream
    let my_type: TokenStream = my_type;

    // defaults to [TokenStream] if _
    static MY_TYPE: [TokenStream] = [T, U, V];
    static MY_TYPE: _ = MY_TYPE;
    // can be set to TokenStream
    static MY_TYPES: TokenStream = T;

    // defaults to TokenStream
    type MyType: TokenStream = T;
    type MyType = MyType;
    // syntactically cannot be set to [TokenStream]

    // defaults to TokenStream if _
    const MY_TYPE: TokenStream = T;
    const MY_TYPE: _ = MY_TYPE;
    // can be set to [TokenStream]
    const MY_TYPES: [TokenStream] = [MY_TYPE];
    const MY_TYPES: [_] = [MY_TYPE];
    // static is the same as const

    type Reportable = Static + Error + Send + Sync + Display + Debug;

    let int_from: [_] = [bool, u8, u16, u32, u64, i8, i16, i32, i64, i128];
    let int_try_from: [_] = [u128, usize, isize];
    let int_try_from_approximate: [_] = [f32, f64] | [] & [];
    let int_maybe_from: [_] = int_from + int_try_from + int_try_from_approximate;

    static int_maybe_from: [_] = int_from + int_try_from + int_try_from_approximate;
    const from: _ = From;

    let int_into: [TokenStream] = [i128];
    let int_try_into: [TokenStream] =
        [BOOL, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize];
    let int_into_approximate: [TokenStream] = [f32, f64];
    let int_maybe_into: [TokenStream] = int_into + int_try_into + int_into_approximate;

    // `let`, `type`, and `const`, 'static' are equivalent
    // they have different suggested capitalizations and unescaped grammar.
    // static too, 'cause why not? Use what makes the most sense for your code.
    let static_lifetime: TokenStream = TokenStream!('static);
    type static_lifetime: TokenStream = TokenStream!(Lifetime static);
    type static_lifetime: TokenStream = Any + 'static;

    trait static_lifetime: TokenStream {
        TokenStream!(Lifetime static);
    }

    let static_lifetime = TokenStream!('static ;);

    let reportable: TokenStream = static_lifetime + join!(E rr or) + Display + Send + Sync;
    let reportable: TokenStream = static_lifetime + Error + Display + Send + Sync;

    // how do we do many at once?
    // for _ in cross!(
    //    x in [1]
    //           y in [2]
    //    z in [3],
    // ) {
    //     let _ = a + b + c + d;
    // }

    // do we want to overload multiplication for cartesian product
    for (x, y, z) in A * B * C {
        let _ = a + b + c + d;
    }

    // and zip can just be
    for (x, y, z) in (A, B, C) {}
    // except i can't cram that into the runtime
    for (x, y, z) in A / B / C {}

    // shoving this all into the runtime would be funny

    // for _ in join! {
    //           x in [1]
    //    y in [2]
    //    z in [3]
    // } {
    //     let _ = a + b + c + d;
    // }

    let output: [TokenStream] = for SomeInt in int_from {
        impl From<SomeInt> for Int {
            fn from(value: SomeInt) -> Int {
                Int(value.try_into().unwrap())
            }
        }
    };

    {
        #[test]
        pub fn main() {
            println!("output");
        }
    }
};
