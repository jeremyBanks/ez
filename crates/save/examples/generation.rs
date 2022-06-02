use {eyre::Result, git2::Repository, save::git2::CommitExt};

pub(crate) fn decode_hex_nibbles(s: impl AsRef<str>) -> (Vec<u8>, Vec<u8>) {
    let mut hex_bytes = s.as_ref().as_bytes();
    if hex_bytes.get(0) == Some(&b'0') && matches!(hex_bytes.get(1), Some(b'x' | b'X')) {
        hex_bytes = &hex_bytes[2..];
    }
    let capacity = (hex_bytes.len() + 1) / 2;
    let mut bytes = Vec::<u8>::with_capacity(capacity);
    let mut mask = Vec::<u8>::with_capacity(capacity);
    let mut buffer_byte: Option<u8> = None;

    for byte in hex_bytes {
        let nibble = match byte {
            b'0'..=b'9' => byte.wrapping_sub(b'0'),
            b'a'..=b'f' => byte.wrapping_sub(b'a' - 10),
            b'A'..=b'F' => byte.wrapping_sub(b'A' - 10),
            b'_' | b' ' | b'\n' | b'\t' | b',' | b';' => continue,
            _ => panic!("Invalid byte {byte:?} ({:?}) in hex input.", *byte as char),
        };

        if let Some(byte) = buffer_byte.take() {
            bytes.push(byte | nibble);
            mask.push(0xFF);
        } else {
            buffer_byte = Some(nibble << 4);
        }
    }

    if let Some(byte) = buffer_byte {
        bytes.push(byte);
        mask.push(0xF0);
    }

    bytes.zip(mask)
}

pub(crate) fn decode_hex_bytes(s: impl AsRef<str>) -> Vec<u8> {
    let s = s.as_ref();
    if s.len() % 2 != 0 {
        panic!("Odd number of digits in hex string.");
    }
    decode_hex_nibbles(s).0
}

#[macro_export(crate)]
macro_rules! hex {
    [$($hex:tt)*] => {
        crate::decode_hex_bytes(stringify!($($hex)*))
    }
}

#[macro_export(crate)]
macro_rules! hex_masked {
    [$($hex:tt)*] => {
        crate::decode_hex_nibbles(stringify!($($hex)*))
    }
}

fn main() -> Result<()> {
    let repo = Repository::open_from_env()?;
    let head = repo.head()?.peel_to_commit()?;
    let tree = head.tree()?;
    let head = &head.id().to_string()[..8];
    let tree = &tree.id().to_string()[..8];

    let revision = 409;
    let generation = 1647;
    let number = 1862;
    println!(
        "
    initial commit:
        message: r0

    first merge of a single parallel commit:
        message: r4 / n5

    first merge of a single commit that could be fast-forwarded instead:
        message: r4 / g5

    typical non-linear head:
        message: r{revision} / g{generation} / n{number}
             id: {head}
           tree: {tree}
    "
    );

    let hex = hex![0x123a2631311531532323524624624624375373572437331131131313131313145678];
    println!("hex: {hex:02x?}");
    let hex_masked = hex_masked![0x123456789];
    println!("hex_masked: {hex_masked:02x?}");

    Ok(())
}
