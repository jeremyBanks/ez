Something with errors.

# Alternatives

- [`fehler`](https://docs.rs/fehler/latest/fehler/) provides a `#[throws]`
  attribute and `throws!()` macro that work similarly to the ones in this
  module, but it doesn't default to `eyre::Report` or add the panicking variant.
