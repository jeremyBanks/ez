# Introducing Doop

_April 1, 2022_

I would like to introduce `doop`–pronounced like "dupe" and spelled like "loop"–a new procedural
macro for local code duplication using a loop-style syntax. `doop` improves on existing options by
providing a more familiar and readable syntax for users.

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

### Attribute-Style (`#[dooped(...)]`)

This is an option to.
