use ez::batteries::doop;

fn main() {
    doop! {
        let Literals = ["a", 'b', "see", 3, 0.0, 10u8];
        let Identifiers = [a, b, see, _3, Trait, Type];
        let Keywords = [fn, loop, let];
        let Paths = [::std, u64, std::collections::HashMap<i8, u64>];
        let Expressions = [1 + 1, println!("test"), if true { false } else { true } ];
        let Parenthesized = [{}, (1, 2), {1, 2, 3}, (true)];
        let Blocks = [{ println!("test"); }, {{ println!("test"); }}];
        let Combined = Literals + Identifiers + Paths + [?];
        let Excluded = Literals - ["a"];

        for X in Combined {
            println!("{}", stringify!(X));
        }

        for $ in [1, 2, 3] + Literals - [0.0, "ignored missing value"] {
            let literal = $;
            println!("{literal}");
        }

        for SomeType in [u8, u16, u32]
        for some_number in [1, 2, 3] {
            let n: SomeType = some_number;
            println!("{n}");
        }

        let Ops = [+, -, /, *];

        for @ in Ops
        for +- in [+, -] {
            println!("{}", 4 @ 2 +- 10);
        }
    }
}
