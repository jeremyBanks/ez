`doop`—spelled like "loop" and pronounced like "dupe"—is a macro for local code
duplication in Rust.

This type of code duplication should usually be avoided in favour of more
conventional language features like traits (or even functions). However for
certain niche cases, those might not be suitable, and this may be the alternative
you need.

## Example

```rust
use doop::doop;

doop! {
    // Aliases may be defined at the top of the macro body using `let`.
    //
    // These can be made up of raw token trees, bracketed and comma-delimited...
    let UnsignedTypes = [u8, u16, u32];
    let SignedTypes = [i8, i16, i32];
    let BinaryOps = [+, -, /];
    // ...and/or we can reference existing aliases:
    let Types = SignedTypes + UnsignedTypes;

    // Each piece of code to be duplicated is indicated with a `for`-loop-style
    // block, optionally with multiple `for` statements for nested repetitions.
    for Type in Types
    for B in BinaryOps {
        let n: Type = 1;
        dbg!(n B n);
    }

    // `for` can refer to aliases and/or bracketed token trees, just like `let`.
    for Type in Types [u128, i128]
    for U in [!, -] {
        let n: Type = 2;
        dbg!(U n);
    }
}
```

### Expanded

```rust
let n: u8 = 1;
dbg!(n + n);

let n: u8 = 1;
dbg!(n - n);

let n: u8 = 1;
dbg!(n * n);

let n: u8 = 1;
dbg!(n / n);

let n: u16 = 1;
dbg!(n + n);

// ...

let n: i16 = 2;
dbg!(- n);

let n: i32 = 2;
dbg!(! n);

let n: i32 = 2;
dbg!(- n);
```

## Alternatives

- [the `duplicate` crate](https://crates.io/crates/duplicate)
- [convoluted use of `macro_rules`](https://stackoverflow.com/q/37752133/1114)
