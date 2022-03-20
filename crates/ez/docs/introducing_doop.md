# Introducing Doop

_April 1, 2022_

I would like to introduce my new crate, [`doop`](https://crates.io/crates/doop). Doop—pronounced
like "dupe" and spelled like "loop"—is a new procedural macro for local code duplication using a
loop-style syntax. Doop improves on existing options by providing a simple and familiar syntax while
retaining enough flexibility to cover most uses.

Doop can be used as the [`doop!{ … }`][self] function-style macro or, in cases where you're
duplicating one [item](https://doc.rust-lang.org/reference/items.html), as the
[`#[dooped( … )]`][self] attribute macro. I'll show examples of the different ways it can be used,
then elaborate on the details of the syntax.

## Examples

> 💡 <em>You can</em> 👉 <em>click</em> 👈 <em>on any example to display its full macro expansion.</em>

<details><summary><strong>Duplicating a single item with a simple replacement:</strong>

```rust
#[dooped(for SomeType in [u8, i8, u16, i16])]
impl MyTrait for SomeType {
    fn some() -> SomeType {
        42
    }
}
```

</summary>

```rust
impl MyTrait for u8 {
    fn some() -> u8 {
        42
    }
}

impl MyTrait for i8 {
    fn some() -> u16 {
        42
    }
}

impl MyTrait for u16 {
    fn some() -> u16 {
        42
    }
}

impl MyTrait for i16 {
    fn some() -> u16 {
        42
    }
}
```

</details><details><summary><strong>Duplicating statements with tuple-style destructuring of replacements:</strong>

```rust
doop! {
    for (name, Type, value) in [
        (four, i64, 2 + 2),
        (platform, &str, "linux"),
        (unit, (), Default::default()),
    ] {
        let name: Type = value;
        println!("{:?}", name);
    }
}
```

</summary>

```rust
let four: i64 = 2 + 2;
println!("{:?}", four);

let platform: &str = "linux";
println!("{:?}", platform);

let unit: () = Default::default();
println!("{:?}", unit);
```

</details><details><summary><strong>Binding lists of replacements using pseudo <code>let</code> </strong>statements:

```rust
doop! {
    let Types = [i8, u8, i16, u16];

    for Type in Types {
        println!("{#:?}", Type::default());
    }

    for Type in Types {
        println!("{#:?}", Type::default() + 1);
    }
}
```

</summary>

```rust
println!("{#:?}", i8::default());
println!("{#:?}", u8::default());
println!("{#:?}", i16::default());
println!("{#:?}", u16::default());

println!("{#:?}", i8::default() + 1);
println!("{#:?}", u8::default() + 1);
println!("{#:?}", i16::default() + 1);
println!("{#:?}", u16::default() + 1);
```

</details><details><summary><strong>Nested iterations:</strong>

```rust
doop! {
    for (name, value) in [(apple, 1), (banana, 2), (cherry, 3)]
    for SUFFIX in [+ name, - 42, * 123, .pow(3)]
    for Type in [i8, u8] {
        let name: Type = value;
        let after = x SUFFIX;
    }
}
```

</summary>

```rust
let apple: i8 = 1;
let after = apple + apple;
let apple: u8 = 1;
let after = apple + apple;
let apple: i8 = 1;
let after = apple - 42;
let apple: u8 = 1;
let after = apple - 42;
let apple: i8 = 1;
let after = apple * 123;
let apple: u8 = 1;
let after = apple * 123;
let apple: i8 = 1;
let after = apple.pow(3);
let apple: u8 = 1;
let after = apple.pow(3);
let banana: i8 = 2;
let after = banana + banana;
let banana: u8 = 2;
let after = banana + banana;
let banana: i8 = 2;
let after = banana - 42;
let banana: u8 = 2;
let after = banana - 42;
let banana: i8 = 2;
let after = banana * 123;
let banana: u8 = 2;
let after = banana * 123;
let banana: i8 = 2;
let after = banana.pow(3);
let banana: u8 = 2;
let after = banana.pow(3);
let cherry: i8 = 3;
let after = cherry + cherry;
let cherry: u8 = 3;
let after = cherry + cherry;
let cherry: i8 = 3;
let after = cherry - 42;
let cherry: u8 = 3;
let after = cherry - 42;
let cherry: i8 = 3;
let after = cherry * 123;
let cherry: u8 = 3;
let after = cherry * 123;
let cherry: i8 = 3;
let after = cherry.pow(3);
let cherry: u8 = 3;
let after = cherry.pow(3);
```

</details><details><summary><strong>Combining lists using <code>+</code>:</strong>

```rust
#[dooped(for SomeType in [u8, i8, u16, i16])]
impl MyTrait for SomeType {
    fn some() -> SomeType {
        42
    }
}
```

</summary>

```rust
impl MyTrait for u8 {
    fn some() -> u8 {
        42
    }
}

impl MyTrait for i8 {
    fn some() -> u16 {
        42
    }
}

impl MyTrait for u16 {
    fn some() -> u16 {
        42
    }
}

impl MyTrait for i16 {
    fn some() -> u16 {
        42
    }
}
```

</details>

## Full Syntax

### Macros

### Lists

### `let` Bindings

### `for` Repetitions

### Operations

#### Add `+`

#### Remove `-`

#### Cartesian Product as Tuples `*`

#### Concatenation/Zipping as Tuples `&`

## Stability and MSRV

The initial release is version `0.1`. Backwards-incompatible changes are not anticipated, but may
happen until version `1.0`.

The minimum supported Rust version is 1.59; if this is ever increased it will be considered a
semver-breaking change.

## Alternatives/Prior Art

## Feedback

Bug reports, requests, questions, and other feedback are welcome
[as GitHub issues](https://github.com/jeremyBanks/ez/issues/new).
