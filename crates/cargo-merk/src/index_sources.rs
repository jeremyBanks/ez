use ez::{prelude::*, throws};

static OFFICIAL: &str = "https://github.com/rust-lang/crates.io-index";
static OFFICIAL_ARCHIVE: &str = "https://github.com/rust-lang/crates.io-index-archive";
static UNOFFICIAL_MIRRORS: (&str, &str) = (
    "https://gitlab.com/rust-lang/crates.io-index",
    "https://gitlab.com/integer32llc/crates.io-index",
);
static EXPERIMENTAL_HEAD: &str = "https://crates.jeremy.ca";
static KNOWN_COMMITS: &[(u64, [u8; 20], &str)] = &[
    (000_000, hex!["a33de1c98898dc1baf541ee2c5162e7baea7c838"], "starting-2014-11-04"),
    (111_423, hex!["9110daee6752e903379f3af955506d6116315273"], "snapshot-2018-09-26"),
    (111_424, hex!["c2c8a0e18edca7d2861cf7af3c40ad9a554ea8c2"], "squashed-2018-09-26"),
    (187_768, hex!["e669e7256d9d00baea377e9f487c0d086ac78c2c"], "snapshot-2019-10-17"),
    (187_769, hex!["5f2e23e89e97bedc21250a4c57216dd2f6182a5e"], "squashed-2019-10-17"),
    (230_660, hex!["f6bccfc6021a2088cb0e89652b9bfcd105c0c2a0"], "snapshot-2020-03-25"),
    (230_661, hex!["1c0a89d50b5ca43c0813181e9dc9f74a8b2e7662"], "squashed-2020-03-25"),
    (276_971, hex!["eb6c4f86a152ee407c7a466327c6a4cbbb92cd7a"], "snapshot-2020-08-04"),
    (276_972, hex!["a10c9c37849a2fff24088f9656ea640d49dddd35"], "squashed-2020-08-04"),
    (315_998, hex!["1b7e17acbb67d41e148ba6dbaf8975f412dc6207"], "snapshot-2020-11-20"),
    (315_999, hex!["46a429eac9f70fd7281922780d7dd42e2fb7ab77"], "squashed-2020-11-20"),
    (387_732, hex!["a5dcd8438da2d8f99e3661a1956afbfb8f026fa0"], "snapshot-2021-05-05"),
    (387_733, hex!["4a4435768950e85c33c0003092ef7740452af85c"], "squashed-2021-05-05"),
    (411_567, hex!["4181c62812c70fafb2b56cbbd66c31056671b445"], "snapshot-2021-07-02"),
    (411_568, hex!["3804ec0c71f6e19dacb274e07d009faf3f106882"], "squashed-2021-07-02"),
    (447_162, hex!["f954048ea7b374a6261fa751710b73981b292048"], "snapshot-2021-09-24"),
    (447_163, hex!["8fe6ce0558479f48e4da8c6e6695f1b7bbc445d0"], "squashed-2021-09-24"),
    (491_842, hex!["94b5429198de77c890839b962228b187f0c25468"], "snapshot-2021-12-21"),
    (491_843, hex!["dcb14b9012f15cf806bd0a61c982e9d6f76a7d63"], "squashed-2021-12-21"),
    (533_011, hex!["ba5efd5ab04919dd77b8a7b8298327c3ce75457e"], "snapshot-2022-03-02"),
    (533_012, hex!["d511f68fa91e266ba7a20b5f37e7a4801423c289"], "squashed-2022-03-02"),
];

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
/// divergence should be alarming, but if the lower priority repositories are
/// stale or unavailable, that might not be.
pub fn head_repos() -> Vec<String> {
    vec![
        OFFICIAL.to_string(),
        local_cargo_index(),
        EXPERIMENTAL_HEAD.to_string(),
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
/// the latest one is really neccessary for verifying the current index state,
/// but the others may be useful when testing the validation logic.
pub fn known_commits() -> SortedMap<u64, [u8; 20]> {
    KNOWN_COMMITS.iter().map(|(index, hash, _)| (*index, *hash)).collect()
}

pub fn known_refs() -> OrderedMap<String, [u8; 20]> {
    KNOWN_COMMITS.iter().map(|(_, hash, name)| (name.to_string(), *hash)).collect()
}
