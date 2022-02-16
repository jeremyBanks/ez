Decorates a `main` entry point function to configure some standard capabilities
such as logging and error reporting, and adds some other conveniences.

## Features

- Pretty formatting for stack traces and other errors using
  [`color-eyre`](https://docs.rs/color-eyre).
- Enables console output for `log` and `tracing` logs.
  - Log levels may be configured with the `RUST_LOG` environment variable (as
    per [`env_logger`](https://docs.rs/env_logger)).
  - By default, the log level for the main crate is set to `debug` in `dev`
    builds and to `info` in `release` builds.
  - By default, the log level is set to `warn` for all other crates.
- Loads `.env` files using [`dotenv`](https://docs.rs/dotenv).
- Includes the error handling behaviour of [`#[throws]`][crate::throws], so
  errors may be propagated with [the `?` operator][std::ops::Try] or returned
  with [`throw!`][crate::throw] in order to end the process with a nonzero exit
  status.
- Optionally, a return type of `u8` or `i32` may be declared and will be used as
  the process' exit status.
- Optionally, arguments `(args: T)` or `(args: T, env: U)` may be declared, and
  will be populated with the command line arguments and environment variables
  respectively.
  - `T` may be [`Vec<String>`] or any other type that implements
    [`FromIterator<String>`].
  - `U` may be [`std::collections::HashMap<String, String>`] or any other type
    that implements [`FromIterator<(String, String)>`][FromIterator].
- If the function is marked as `async`, it will be run in the single-threaded
  [`tokio`](https://docs.rs/tokio) runtime.
  - This is like adding the `#[tokio::main]` macro. (We only don't actually use
    `tokio::main` internally because of procedural macro hygiene limitations.)
