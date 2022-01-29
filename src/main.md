This module provides a `#[main]` macro, intended for use on your entry-point
`main` function.

It has the following effects:

- enables [`::color-eyre`] for nicely-formatted colored error backtraces.
  - we default to `RUST_LIB_BACKTRACE=1` in `debug` builds
- enables console output for log messages from `tracing` and `log`.
  - we default to log level `WARN` for imported crates
  - we default to log level `INFO` for the current crate
- wraps main in a `Result` with an `eyre::Report` to enable use of `?` to
  propagate top-level errors into a crash.
- `main()` can optionally be defined with primitive integer return type (or an
  `ez::Int`). if so, the process will use `main()`'s return value as its exit
  status, as in C. (This ensures that all destructors on the stack have had a
  chance to run, which is not the case when calling `std::process::exit()`
  directly.)
- `main()` can optionally be defined to take one or two arguments:
  - `args` must be a `&[&str]`. If any of the arguments are not valid UTF-8,
    a warning will be logged and invalid segments will be replaced with the
    replacement character `'�'`.
  - `env` must be a `&HashMap<&str, &str>`. If any keys or values are not valid
    UTF-8, an warning will be logged and they will be excluded from the list.
