# Introducing Doop

_April 1, 2022_

I would like to introduce **Doop**–pronounced like "dupe" and spelled like "loop"–a new procedural
macro for local code duplication using a loop-style syntax. Doop improves on existing options by
providing a more familiar and readable syntax for developers.

Let me take you through a few examples.

## Examples

```rust
use doop::doop;

doop! {
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
}
```

## Details

They're kind-of neat.

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
