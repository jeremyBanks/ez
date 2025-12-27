use doop::doop;

// Test the simplest possible case
doop! {
    for Type in [u8, i16] {
        fn test_Type() -> Type {
            42 as Type
        }
    }
}

#[test]
fn test_expansion() {
    // If doop worked, these should exist:
    let _x = test_u8();
    let _y = test_i16();
}
