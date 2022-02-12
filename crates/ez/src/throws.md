Decorates a function so that it implicitly wraps the result in [`Result::Ok`],
unless a [`Result::Err`] is propagated with [the `?` operator][std::ops::Try] or
returned with [the `throw!` macro][crate::throw].

By default, `#[throws]` will use [`eyre::Report`] for the error value. This is a
dynamic error type that's useful for error reporting in application code.
However, if you're writing library code you may want to use a more specific
error type, and one may be specified as an argument to this macro, such as
`#[ez::throws(std::num::ParseIntError)]`.

The [`throw!`][crate::throw] macro will be automatically imported in the body of
functions decorated with `#[throws]`.

## Example 1

```rust
#[throws]
fn expects_ten(n: i32) {
    if n != 10 {
        throw!("expected 10, but got {n}");
    }
}
```

This is expanded out into something equivalent to:

```rust
# use ez::__::eyre;
fn expects_ten(n: i32) -> Result<(), eyre::Report> {
    if n != 10 {
        return Err(eyre::Report::msg(format!("expected 10, but got {n}")))
    }

    Ok(())
}
```

## Example 2

```rust
#[throws(std::num::ParseIntError)]
fn check_ten(n: &str) -> bool {
    let n: i32 = n.parse()?;

    n == 10
}
```

This is expanded out into something equivalent to:

```rust
fn check_ten(n: &str) -> Result<bool, std::num::ParseIntError> {
    let n: i32 = n.parse()?;

    Ok(n == 10)
}
```
