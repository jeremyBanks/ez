#[doop::doopin]
const implementations: X = {
    let BOOL: TokenStream = BOOL;

    let int_from: [TokenStream] = [BOOL, u8, u16, u32, u64, i8, i16, i32, i64, i128];
    let int_try_from: [TokenStream] = [u128, usize, isize];
    let int_try_from_approximate: [TokenStream] = [f32, f64] + [] - [];
    let int_maybe_from: [TokenStream] = int_from + int_try_from + int_try_from_approximate;

    let int_into: [TokenStream] = [i128];
    let int_try_into: [TokenStream] = [BOOL, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize];
    let int_into_approximate: [TokenStream] = [f32, f64];
    let int_maybe_into: [TokenStream] = int_into + int_try_into + int_into_approximate;

    let static_lifetime: TokenStream = X!{ 'static };
    let reportable: TokenStream = static_lifetime + Error + Display + Send + Sync;
    let reportable: TokenStream = static_lifetime + Error + Display + Send + Sync;

    {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct Int(i128);

        2 + 2;
    }

    for int in int_from {
        impl From<int> for Int {
            fn from(value: int) -> Int {
                Int(value.try_into().unwrap())
            }
        }
    }
};

#[test]
fn test() {
    println!("{}", implementations);
}
