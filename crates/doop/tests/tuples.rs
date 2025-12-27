use doop::doop;

// Test tuple destructuring
trait GetValue {
    fn get_value() -> i32;
}

doop! {
    for (Type, value) in [(u8, 1), (i16, 2), (u32, 3)] {
        impl GetValue for Type {
            fn get_value() -> i32 {
                value
            }
        }
    }
}

#[test]
fn test_tuple_destructuring() {
    assert_eq!(u8::get_value(), 1);
    assert_eq!(i16::get_value(), 2);
    assert_eq!(u32::get_value(), 3);
}

// Test with 3-tuple
trait Triple {
    fn describe() -> &'static str;
}

doop! {
    for (T, name, val) in [(u8, "byte", 8), (u16, "word", 16), (u32, "dword", 32)] {
        impl Triple for T {
            fn describe() -> &'static str {
                name
            }
        }
    }
}

#[test]
fn test_triple() {
    assert_eq!(u8::describe(), "byte");
    assert_eq!(u16::describe(), "word");
    assert_eq!(u32::describe(), "dword");
}
