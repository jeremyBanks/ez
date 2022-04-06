# What crates should be in a "batteries-included" package for Rust?

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
