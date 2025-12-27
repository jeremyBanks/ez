use doop::doop;

// Test with impls where Type is used as a standalone identifier
trait Double {
    fn double(self) -> Self;
}

doop! {
    for Type in [u8, i16, u32] {
        impl Double for Type {
            fn double(self) -> Type {
                self * 2
            }
        }
    }
}

#[test]
fn test_trait_impl() {
    assert_eq!(5u8.double(), 10u8);
    assert_eq!(5i16.double(), 10i16);
    assert_eq!(5u32.double(), 10u32);
}

// Test with simple types
trait AsNumber {
    fn as_number(&self) -> i32;
}

doop! {
    for T in [u8, u16, u32] {
        impl AsNumber for T {
            fn as_number(&self) -> i32 {
                *self as i32
            }
        }
    }
}

#[test]
fn test_as_number() {
    let a: u8 = 5;
    let b: u16 = 10;
    let c: u32 = 15;

    assert_eq!(a.as_number(), 5i32);
    assert_eq!(b.as_number(), 10i32);
    assert_eq!(c.as_number(), 15i32);
}
