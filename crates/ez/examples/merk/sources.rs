use ez::batteries::hex;

pub static URL: &str = "https://github.com/rust-lang/crates.io-index";

pub static TRUSTED_COMMITS: &[(u64, [u8; 20], &str)] = &[
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

pub static ALL_HEADS: &[&str] =
    &[URL, "https://gitlab.com/rust-lang/crates.io-index", "https://crates.jeremy.ca/"];

pub static CANONICAL_ARCHIVE: &str = "https://github.com/rust-lang/crates.io-index-archive";

pub static ALL_ARCHIVES: &[&str] = &[CANONICAL_ARCHIVE];
