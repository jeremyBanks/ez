# Introducing `ez::main`, `ezrs`, and `batteries`

_April 3, 2022_

I would like to introduce my new crate, [`erro`](https://crates.io/crates/erro).

## Examples

> 💡 <em>tip: You can</em> 👉 <em>click</em> 👈 <em>on any example to display its full macro
> expansion.</em>

<details><summary><strong>Duplicating a single item with a simple replacement:</strong>

```rust
#!/usr/bin/env ezrs

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
