use {
    crate::*,
    ::{
        sha1::{Digest, Sha1},
        std::fmt::Write,
    },
};

/// Returns the hexadecimal SHA-1 digest of bytes as git would for a blob
/// (file).
///
/// This should make the file hash consistent with what you see in git.
pub fn git_file_sha1(content: &[u8]) -> String {
    let mut hasher = Sha1::default();
    hasher.update(b"blob ");
    hasher.update(content.len().to_string());
    hasher.update(b"\0");
    hasher.update(content);
    let result = hasher.finalize();
    let mut hex = String::with_capacity(40);
    for byte in result {
        write!(hex, "{:02x}", byte).unwrap();
    }
    hex
}
