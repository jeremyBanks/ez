#[ez::ly]
fn main() {
    doop! {
        let Literals = [
            'a', 2, "c", 4.0
        ];

        let Pairs = [
            ("see", "saw"),
            ("ego", 2),
            ("fee", 4.0),
            ("foe", "fum")
        ];
        // XXX: Should trailing commas be ignored, by default?
        // If you really want an empty trailing element, it needs to be explicit, like `[, []]`
        // Or maybe we should assume empty items are errors in all cases, unless they're explicit.
        // Our error message can suggest appropriate replacements.

        let Types = [
            u32,
            u16,
            u64
        ];

        static {
            trait MyAdd<Other> {}
        }

        for T1 in Types
        for T2 in Types - [T1] {
            impl MyAdd<T2> for T1 {}
        }

        static {
            use std::ops::{Mul, Div};
        }

        for (Trait, method, OP) in [
            (std::ops::Add, add, +),
            (::core::ops::Sub, sub, -),
            (Mul, mul, *),
            (Div, div, /)
        ] {
            let a = 4;
            let b = 6;
            let as_op = a OP b;
            let as_method = Trait::method(a, b);
            assert_eq!(as_op, as_method);
        }

        for X in [[1], (2), {3}] + {[1], (2), {3}}  {
            println!("{:10} {:10?}", stringify!(X), X);
        }

        static {
            println!("dedupe!");
        }

        let Identifiers = [a, b, see, _3, Trait, Type];
        let Keywords = [fn, loop, let];
        let Paths = [::std, u64, std::collections::HashMap<i8, u64>];
        let Expressions = [1 + 1, println!("test"), if true { false } else { true } ];
        let Parenthesized = [{}, (1, 2), {1, 2, 3}, (true)];
        let Blocks = [{ println!("test"); }, {{ println!("test"); }}];
        let Combined = Literals + Identifiers + Paths + [?];
        let Excluded = Literals - ["a"];

        for X in Blocks {
            println!("{}", stringify!(X));
        }

        // Destructuring is supported, but only for tuple-style items
        // (wrapped in parentheses, delimited by commas). All items must have the same number
        // of items/commas, but you can use `_` to ignore an item.
        for (NAME, value) in [(a, 2), (b, 3), (c, 4)] {
            let name = stringify!(NAME);
            println!("{} = {}", name, value);
        }

        for X in [1, 2, 3] + Literals - [0.0, "ignored missing value"] {
            let literal = X;
            println!("{literal}");
        }

        for SomeType in [u8, u16, u32]
        for some_number in [1, 2, 3] {
            let n: SomeType = some_number;
            println!("{n}");
        }

        let Ops = [+, -, /, *];

        for (Trait, method, OP) in [
            (std::ops::Add, add, +),
            (::core::ops::Sub, sub, -),
            (Mul, mul, *),
            (Div, div, /)
        ] {
            let a = 4;
            let b = 6;
            assert_eq!(a OP b, Trait::method(a, b));
        }
    }

    trait T2 {}
    #[dooped(for T1 in [u8, i16, u32])]
    impl T2 for T1 {}
}
