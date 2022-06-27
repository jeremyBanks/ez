use ez::{prelude::*, throws};

static OFFICIAL: &str = "https://github.com/rust-lang/crates.io-index";
static OFFICIAL_ARCHIVE: &str = "https://github.com/rust-lang/crates.io-index-archive";
static UNOFFICIAL_MIRRORS: (&str, &str) = (
    "https://gitlab.com/rust-lang/crates.io-index",
    "https://gitlab.com/integer32llc/crates.io-index",
);

#[derive(Debug, Clone)]
struct KnownCommit {
    rev: u64,
    sha: [u8; 20],
    tree: [u8; 20],
    tag: &'static str,
}

impl KnownCommit {
    pub const fn new(rev: u64, sha: [u8; 20], tree: [u8; 20], tag: &'static str) -> KnownCommit {
        KnownCommit { rev, tree, sha, tag }
    }
}

static KNOWN_COMMITS: &[KnownCommit] = &[
    KnownCommit::new(
        000_000,
        hex!["a33de1c98898dc1baf541ee2c5162e7baea7c838"],
        hex!["48266644de27fa08c9535e834a986dea9266273c"],
        "starting-2014-11-04",
    ),
    KnownCommit::new(
        111_423,
        hex!["9110daee6752e903379f3af955506d6116315273"],
        hex!["30327f697fc8f3db65e7455edcd58b32e2080cfa"],
        "snapshot-2018-09-26",
    ),
    KnownCommit::new(
        111_424,
        hex!["c2c8a0e18edca7d2861cf7af3c40ad9a554ea8c2"],
        hex!["30327f697fc8f3db65e7455edcd58b32e2080cfa"],
        "squashed-2018-09-26",
    ),
    KnownCommit::new(
        187_768,
        hex!["e669e7256d9d00baea377e9f487c0d086ac78c2c"],
        hex!["7d57fc9cab3df3e7efcdb4509c677d9832d307f1"],
        "snapshot-2019-10-17",
    ),
    KnownCommit::new(
        187_769,
        hex!["5f2e23e89e97bedc21250a4c57216dd2f6182a5e"],
        hex!["7d57fc9cab3df3e7efcdb4509c677d9832d307f1"],
        "squashed-2019-10-17",
    ),
    KnownCommit::new(
        230_660,
        hex!["f6bccfc6021a2088cb0e89652b9bfcd105c0c2a0"],
        hex!["55055b0ab72750a3aebead5523a5fb2c65c7e90b"],
        "snapshot-2020-03-25",
    ),
    KnownCommit::new(
        230_661,
        hex!["1c0a89d50b5ca43c0813181e9dc9f74a8b2e7662"],
        hex!["55055b0ab72750a3aebead5523a5fb2c65c7e90b"],
        "squashed-2020-03-25",
    ),
    KnownCommit::new(
        276_971,
        hex!["eb6c4f86a152ee407c7a466327c6a4cbbb92cd7a"],
        hex!["ba5057b162f9b9810821ec74bc0840bdcde763e4"],
        "snapshot-2020-08-04",
    ),
    KnownCommit::new(
        276_972,
        hex!["a10c9c37849a2fff24088f9656ea640d49dddd35"],
        hex!["ba5057b162f9b9810821ec74bc0840bdcde763e4"],
        "squashed-2020-08-04",
    ),
    KnownCommit::new(
        315_998,
        hex!["1b7e17acbb67d41e148ba6dbaf8975f412dc6207"],
        hex!["0c2381def0e0236553b50271e52a2a89e8de0f7a"],
        "snapshot-2020-11-20",
    ),
    KnownCommit::new(
        315_999,
        hex!["46a429eac9f70fd7281922780d7dd42e2fb7ab77"],
        hex!["0c2381def0e0236553b50271e52a2a89e8de0f7a"],
        "squashed-2020-11-20",
    ),
    KnownCommit::new(
        387_732,
        hex!["a5dcd8438da2d8f99e3661a1956afbfb8f026fa0"],
        hex!["f862d76da3bd7968cfbe95feb1745a8ab0b5b84b"],
        "snapshot-2021-05-05",
    ),
    KnownCommit::new(
        387_733,
        hex!["4a4435768950e85c33c0003092ef7740452af85c"],
        hex!["f862d76da3bd7968cfbe95feb1745a8ab0b5b84b"],
        "squashed-2021-05-05",
    ),
    KnownCommit::new(
        411_567,
        hex!["4181c62812c70fafb2b56cbbd66c31056671b445"],
        hex!["6783c58d564b6f8d20948e7e0239c1e8aec8f06a"],
        "snapshot-2021-07-02",
    ),
    KnownCommit::new(
        411_568,
        hex!["3804ec0c71f6e19dacb274e07d009faf3f106882"],
        hex!["6783c58d564b6f8d20948e7e0239c1e8aec8f06a"],
        "squashed-2021-07-02",
    ),
    KnownCommit::new(
        447_162,
        hex!["f954048ea7b374a6261fa751710b73981b292048"],
        hex!["c4cb16b8e5b03971f2cea4e0e775c93221ccfa40"],
        "snapshot-2021-09-24",
    ),
    KnownCommit::new(
        447_163,
        hex!["8fe6ce0558479f48e4da8c6e6695f1b7bbc445d0"],
        hex!["c4cb16b8e5b03971f2cea4e0e775c93221ccfa40"],
        "squashed-2021-09-24",
    ),
    KnownCommit::new(
        491_842,
        hex!["94b5429198de77c890839b962228b187f0c25468"],
        hex!["05e8571a7c60bb688a6a799ed902f7c7d8543b12"],
        "snapshot-2021-12-21",
    ),
    KnownCommit::new(
        491_843,
        hex!["dcb14b9012f15cf806bd0a61c982e9d6f76a7d63"],
        hex!["05e8571a7c60bb688a6a799ed902f7c7d8543b12"],
        "squashed-2021-12-21",
    ),
    KnownCommit::new(
        533_011,
        hex!["ba5efd5ab04919dd77b8a7b8298327c3ce75457e"],
        hex!["fd9e19df4dd640ead086e9fb544c3e59cacf9853"],
        "snapshot-2022-03-02",
    ),
    KnownCommit::new(
        533_012,
        hex!["d511f68fa91e266ba7a20b5f37e7a4801423c289"],
        hex!["fd9e19df4dd640ead086e9fb544c3e59cacf9853"],
        "squashed-2022-03-02",
    ),
];

/*
    parents: both head and tail commits with same tree
    author & committer: bors <bors@rust-lang.org>
    timestamps: 1 second greater than the max of committer and author timestamps of all parents, but UTC
    message: <none?>

    a0---------------a1------------------a3-------
   /                / /                 / /
  r0--r1--...--r1000  r1001--...---r2000 r2001----
*/

pub fn local_cargo_index() -> String {
    cargo_home()
        .unwrap()
        .tap_mut(|path| path.push("registry/index/github.com-1ecc6299db9ec823/.git"))
        .into_os_string()
        .into_string()
        .unwrap()
}

/// A list of repositories whose heads should all track the linear crates.io
/// index. These should be ordered from most-trusted to least-trusted. Any
/// forking should be alarming.
pub fn head_repos() -> Vec<String> {
    vec![
        OFFICIAL.to_string(),
        local_cargo_index(),
        UNOFFICIAL_MIRRORS.0.to_string(),
        UNOFFICIAL_MIRRORS.1.to_string(),
    ]
}

/// A list of repositories to be pulled from when looking for a missing commit,
/// in the order they should be tried.
pub fn data_repos() -> Vec<String> {
    vec![
        local_cargo_index(),
        OFFICIAL.to_string(),
        OFFICIAL_ARCHIVE.to_string(),
        UNOFFICIAL_MIRRORS.0.to_string(),
        UNOFFICIAL_MIRRORS.1.to_string(),
    ]
}

/// A hard-coded list of known index commit IDs, keyed by their corresponding
/// index (in the other sense) in the linear history of all index (in the first
/// sense) commits.
///
/// These are trusted absolutely; nothing should ever conflict with them. Only
/// the latest one is really necessary for verifying the current index state,
/// but the others may be useful when testing the validation logic.
pub fn known_commits() -> SortedMap<u64, [u8; 20]> {
    KNOWN_COMMITS.iter().map(|commit| (commit.rev, commit.sha)).collect()
}

pub fn known_refs() -> OrderedMap<String, [u8; 20]> {
    KNOWN_COMMITS.iter().map(|commit| (commit.tag.to_string(), commit.sha)).collect()
}
