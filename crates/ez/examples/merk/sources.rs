pub static CANONICAL_HEAD: &str = "https://github.com/rust-lang/crates.io-index";
pub static CANONICAL_ARCHIVE: &str = "https://github.com/rust-lang/crates.io-index-archive";

pub static ALL_HEADS: &[&str] = &[
    CANONICAL_HEAD,
    "https://gitlab.com/rust-lang/crates.io-index",
    "https://crates.jeremy.ca/",
];

pub static CANONICAL_REPOS: &[&str] = &[CANONICAL_HEAD, CANONICAL_ARCHIVE];
