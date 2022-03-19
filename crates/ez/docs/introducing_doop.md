# Introducing Doop

_April 1, 2022_

I would like to introduce my new crate, [`doop`](https://crates.io/crates/doop). Doop–pronounced
like "dupe" and spelled like "loop"is a new procedural macro for local code duplication using a
loop-style syntax. Doop improves on existing options by providing a more familiar and readable
syntax for developers.

Let's start with a hypothetical integer wrapper type, which we only to allow to be set to odd
values.

```rust
use doop::{doop, dooped};

#[Derive(Debug, Clone)]
struct OddInteger(i164);

#[derive(Debug, Clone)]
struct IsNotOddError;

impl OddInteger {
    pub fn try_new(integer: i64) -> Result<OddInteger, IsNotOddError> {
        if integer % 2 == 1 {
            Ok(OddInteger(other))
        } else {
            Err(IsNotOddError)
        }
    }
}
```

We could implement [`TryFrom`] for some of the built-in integer types using the `doop!` macro's
for-loop-style syntax, like this:

```rust
doop! {
    for IntType in [u8, i8, u16, i16, u32, i32, i64] {
        impl TryFrom<IntType> for OddInteger {
            type Error = IsNotOddError;
            fn try_from(other: IntType) -> Result<OddInteger, IsNotOddError> {
                OddInteger::try_new(i64::from(other))
            }
        }
    }
}
```

<details>
<summary>
## Alternative
</summary>

Hello world

</details>

Our code block will be copied `IntType` will be

```rust
    for (Name, Inner) in [ (Int, i64), (Float, f64), (Bytes, Vec<u8>) ] {
        pub struct Name {
            inner: Inner
        }

        impl From<Inner> for Name {
            fn from(inner: Inner): Name {
                Name { inner }
            }
        }

        impl From<Name> for Inner {
            fn from(x: Name): Inner {
                x.inner
            }
        }
    }
```

## Details

They're kind-of neat.

## Operations

Precedence is all left-to-right. Use parentheses. If you need something more specific, you need to
use let expressions.

Cartesian product using `*`, which you can then exclude specific ones from. Repetition per-item
using * with an integer? So you can do like

let Types = [u8, u16, u32]; let Pairs = (Types * Types) - (Types * 2)

Yeah, I like this.

### Attribute-Style

If you are duplicating a single item, you may use the `#[dooped(...)]` attribute form instead.

```rust
use doop::dooped;

struct MyType(u16);

#[dooped(for Other in [u8, u16, u32, u64])]
impl From<Other> for MyType {
    fn from(x: Other) -> MyType {
        MyType(x.into())
    }
}
```
