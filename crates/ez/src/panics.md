Functions with `#[ez::panic]` will [`panic!`] with an [`eyre::Report`] if an
error is returned, either through use of the [`throw!`] macro or [the `?`
operator][std::ops::Try].
