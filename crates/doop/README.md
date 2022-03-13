`doop`—spelled like "loop" and pronounced like "dupe"—is a macro for local code duplication in Rust,
using a loop-style syntax.

This type of code duplication should usually be avoided in favour of more conventional language
features like traits (or even functions). However for certain niche cases, those might not be
suitable, and this may be the alternative you need.

## Example

```rust
use doop::doop;

doop! {}
```

### Expanded

```rust
```

## Alternatives

- [the `duplicate` crate](https://crates.io/crates/duplicate)
- [convoluted use of `macro_rules`](https://stackoverflow.com/q/37752133/1114)
