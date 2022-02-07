Returns from the enclosing function with a [`Result::Err`].

This will contain either a provided error object, or an error message (if the
first argument is a formatting string literal). Error messages are formatted
into [`eyre::Report`]s using [`eyre::eyre!`], so you'll get an error if you try
to use an error message in a function that expects a more-specific error type in
its [`Result`].
