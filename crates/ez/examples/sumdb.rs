// https://github.com/rust-lang/crates.io-index/
// https://github.com/rust-lang/crates.io-index-archive/

// unofficial mirror: https://gitlab.com/rust-lang/crates.io-index

// How do we verify the log?

// Internal consistency: linear (single parent).
// Each new tree's initial commit points to the last commit of the previous index.
// No commits may change or remove checksums.
// Ensure that all mirrors are consistent, or rather, reject anything that
// hasn't been copied to other mirrors.
// The number of commits (and ergo, versions) is under a million, so we don't need
// to get too cute since all we need to keep as state is (name, version) -> (checksum),
// which will be pretty minimally small if we throw it in sled or something.

#[ez::ly]
pub fn main() {
    let r = git2::Repository::open_from_env()?;
}
