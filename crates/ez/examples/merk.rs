use crossterm::style::Stylize;

#[ez::ly]
pub fn main() {
    let r = git2::Repository::open_from_env()?;

    // Step 1: fetch all index mirrors
    //         If this is our first fetch, verify that the only root commit
    //         is the expected commit, a33de1c98898dc1baf541ee2c5162e7baea7c838,
    //         and that it includes some recent-as-of-time-of-publication
    //         commit, such as d3309ab55b6adc1151b1d1004ff23e9240d55279.
    //
    // Step 2: verify index/mirrors are consistent with each other and with
    //         our previous local head, and that none of the intermediate commits
    //         violate any constraints:
    //          (a) single parent, or no parents if squashed, but squashes should still
    //              include a reference to the effective parent in the messages.
    //          (b) no checksums should be changed.
    //          (c) no versions should be removed
    //              (this probably needs to allow some exceptions, as long as no new
    //               versions are added with conflicting checksums.)
    //
    //         The local state we keep will probably be a sled-or-similar database
    //         with all { (name, version) -> checksum } pairs and our latest head
    //         commit ID. If possible, we could then truncate all git history
    //         before the HEAD.
    //
    // Step 3: drop any git history we don't need
    //
    // Step 4: read Cargo.lock file to identify all dependencies of current
    //         project, or quit if no current project.
    //
    // Step 5: for each dependency, find its files and...
    //
    // Step 6: verify their checksum (even though cargo already should have)
    //
    // Step 7: if it has a repository in its Cargo.toml, fetch that.
    //
    // Step 8: if it has a commit tagged in the repository, find that commit,
    //         otherwise log a warning but try to find a commit whose tag is
    //         the version number, with or without a prefix v, maybe.
    //
    // Step 9: if we have a release commit, check for any files in the released
    //         bundle which do not match those in the git repository. The obvious
    //         exception is the normalization of Cargo.toml, but we should at
    //         least verify that they are semantically equivalent except for
    //         the expected variations.

    // Brainstorming:
    // - Maybe we should have a way to record the index version that was
}

pub static INDEX_CANONICAL_REPO: &str = "https://github.com/rust-lang/crates.io-index";

pub static INDEX_HEAD_ORACLES: [&str] = [
    "https://crates.jeremy.ca/index"
];

pub static INDEX_HEAD_REPOS: [&str] = [
    INDEX_CANONICAL_REPO,
    "https://gitlab.com/rust-lang/crates.io-index",
];

pub static INDEX_ARCHIVE_REPOS: [&str] = [
    "https://github.com/rust-lang/crates.io-index-archive",
];
