Functions with `#[ez::throws]` will return an [`Err`] if an error is returned,
either through use of the [`throw!`] macro or [the `?` operator][std::ops::Try].
A specific error type may be specified, such as in
`#[ez::throws(std::num::ParseIntError)]`, but by default an [`eyre::Report`]
will be used.
