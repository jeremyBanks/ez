# Introducing Doop

_April 1, 2022_

I would like to introduce my new crate, [`doop`](https://crates.io/crates/doop). Doop—pronounced
like "dupe" and spelled like "loop"—is a new procedural macro for local code duplication using a
loop-style syntax. Doop improves on existing options by providing a more simple and familiar syntax,
while retaining enough flexibility to cover most uses.

Doop can be used as the [`doop!{ … }`][self] function-style macro or, in cases where you're
duplicating one [item](https://doc.rust-lang.org/reference/items.html), as the
[`#[dooped( … )]`][self] attribute macro.

## Examples

```rust
trait MyTrait: Default {
    fn some() -> Self;
}

#[dooped(for SomeType in [u8, i8, u16, i16, u32, i32, i64])]
impl MyTrait for SomeType {
    fn some() -> SomeType {
        42
    }
}
```

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

## Feedback

Bug reports, requests, questions, and other feedback are welcome
[as GitHub issues](https://github.com/jeremyBanks/ez/issues/new).
