use doop::doop;

#[test]
fn test_basic_loop() {
    doop! {
        for Type in [u8, i8, u16, i16] {
            fn test_Type() -> Type {
                42 as Type
            }
        }
    }

    // These functions should now exist
    assert_eq!(test_u8(), 42u8);
    assert_eq!(test_i8(), 42i8);
    assert_eq!(test_u16(), 42u16);
    assert_eq!(test_i16(), 42i16);
}

#[test]
fn test_impl_generation() {
    trait MyTrait {
        fn value() -> i32;
    }

    struct TypeA;
    struct TypeB;
    struct TypeC;

    doop! {
        for (T, val) in [(TypeA, 1), (TypeB, 2), (TypeC, 3)] {
            impl MyTrait for T {
                fn value() -> i32 {
                    val
                }
            }
        }
    }

    assert_eq!(TypeA::value(), 1);
    assert_eq!(TypeB::value(), 2);
    assert_eq!(TypeC::value(), 3);
}
