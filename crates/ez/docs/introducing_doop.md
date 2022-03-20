# Introducing Doop

_April 1, 2022_

I would like to introduce my new crate, [`doop`](https://crates.io/crates/doop). Doop—pronounced
like "dupe" and spelled like "loop"—is a new procedural macro for local code duplication using a
loop-style syntax. Doop improves on existing options by providing a more simple and familiar syntax,
while retaining enough flexibility to cover most uses.

Doop can be used as the [`doop!{ … }`][self] function-style macro or, in cases where you're
duplicating one [item](https://doc.rust-lang.org/reference/items.html), as the
[`#[dooped( … )]`][self] attribute macro. I'll show examples of the different ways it
can be used, then elaborate on the details of the syntax.

## Examples

You can click any example to display its full macro expansion below.

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

</details><br /><details><summary><strong>Duplicating statements with tuple-style destructuring of replacements:</strong>

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

</details><br /><details><summary><strong>Binding lists of replacements using pseudo <code>let</code> statements:</strong>

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

```

</details><br /><details><summary><strong>Nested iterations:</strong>

```rust
doop! {
    for value in [1, 2, 3]
    for SUFFIX in [+ x, - x, * x, .pow(x)]
    for Type in [i8, u8] {
        let x: Type = value;
        let y = x SUFFIX;
    }
}
```
</summary>

```rust

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
