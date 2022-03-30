# What crates should be in a "batteries-included" package for Rust?

Rust's standard library is very high-quality, but it isn't huge. It doesn't need to be, because
Cargo makes it easy to use third-party crates. However, for users who are new to the language, it
can be a source of friction if they need to repeatedly find and evaluate crates for what might be
built-in functionality in more "batteries-included" languages such as Python.

I've registered the crate name **`batteries`** and will use it to re-export a collection of
broadly-useful crates, to help improve discoverability for new users. **What do you think should be
included?**

There have been several past threads about "de-facto standard" or "must use" crates, but most of
them are old and some of their suggestions have fallen out of use. I've created a rough list of some
potential inclusions below, but would appreciate any feedback on items that you think would be
important to include (or exclude!).

- **strings:** `regex`, `unicode-segmentation`
- **bytes:** `bytes`, `hex`, `base64`
- **math:** `num`, `rand`
- **filesystem:** `walkdir`, `glob`
- **benchmarking:** `criterion`
- **logging:** `tracing`<sub>+`log`</sub>, `tracing-subscriber`<sub>+`env-filter`, `json`,
  `parking_lot`</sub>
- **error handling:** `thiserror`, `eyre`, `backtrace`, `tracing_error`, `color_eyre`
- **serialization:** `serde`<sub>+`derive`</sub>, `serde_json`, `toml`, `bincode`, `csv`
- **concurrency:** `rayon`, `dpc-pariter`, `crossbeam`, `parking_lot`
- **async:** `tokio`<sub>+`full`, `parking_lot`</sub>
- **macros:** `paste`, `proc_macro2`, `quote`
- **networking:** `socket2`, `hyper`, `reqwest`
- **utilities:** `tap`, `itertools`, `lazy_static`, `once_cell`
- **command lines:** `clap`<sub>+`derive`</sub>, `crossterm`
- **cryptography:** `digest`, `sha-1`, `sha2`, `sha3`, `blake3`, `md-5`
- **data structures:** `indexmap`, `petgraph`, `bumpallo`, `typed-arena`, `anymap`
- **for Go devs:** `scopeguard`
- **for C devs:** `nix`, `bitflags`, `cfg-if`
- **for Haskell devs:** `either`
