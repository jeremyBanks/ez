A variation of [the `#[throws]` attribute][crate::throws] that creates two
copies of the decorated function. One will use the function name and signature
as-written, panicking if an error is encountered, while the other will add a
`try_` prefix to the function name and return a `Result` instead.
