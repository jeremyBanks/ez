use {eyre::Result, git2::Repository, save::git2::CommitExt};

pub(crate) fn decode_hex_nibbles(s: impl AsRef<str>) -> (Vec<u8>, impl Iterator<Item = u8>) {
    let mut hex_bytes = s.as_ref().as_bytes();
    if hex_bytes.get(0) == Some(&b'0') && matches!(hex_bytes.get(1), Some(b'x' | b'X')) {
        hex_bytes = &hex_bytes[2..];
    }
    let capacity = (hex_bytes.len() + 1) / 2;
    let mut bytes = Vec::<u8>::with_capacity(capacity);
    let mut buffer_byte: Option<u8> = None;

    for byte in hex_bytes {
        let nibble = match byte {
            b'0'..=b'9' => byte.wrapping_sub(b'0'),
            b'a'..=b'f' => byte.wrapping_sub(b'a' - 10),
            b'A'..=b'F' => byte.wrapping_sub(b'A' - 10),
            b'_' | b' ' | b'\n' | b'\t' | b',' | b';' | b'"' | b'\'' => continue,
            _ => panic!("Invalid byte {byte:?} ({:?}) in hex input.", *byte as char),
        };

        if let Some(byte) = buffer_byte.take() {
            bytes.push(byte | nibble);
        } else {
            buffer_byte = Some(nibble << 4);
        }
    }

    let mask_full_bytes = std::iter::repeat(0xFF).take(bytes.len());
    let mask: Box<dyn Iterator<Item = u8>>;

    if let Some(byte) = buffer_byte {
        bytes.push(byte);
        let mask_half_byte = std::iter::once(0xF0);
        mask = Box::new(mask_full_bytes.chain(mask_half_byte))
    } else {
        mask = Box::new(mask_full_bytes);
    }

    (bytes, mask)
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
    let head_str = &head.id().to_string()[..8];
    let tree_str = &tree.id().to_string()[..8];

    let revision = 204;
    let generation = head.generation_number();
    let number = 719;
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
                id: {head_str}
              tree: {tree_str}
        "
    );

    let hex = hex![4b825dc642cb6eb9a060e54bf8d69288fbee4904];
    println!(
        "hex:
        {hex:02x?}
    "
    );

    let hex_masked = hex_masked![4b825dc642cb6eb9a060e54bf8d69288fbee4904];
    println!(
        "hex_masked:
        {:02x?}
        {:02x?}
    ",
        hex_masked.0,
        hex_masked.1.collect::<Vec<_>>()
    );

    let hex_masked = hex_masked![4b825dc642cb6eb9a060e54bf8d69288fbee49045];
    println!(
        "hex_masked:
        {:02x?}
        {:02x?}
    ",
        hex_masked.0,
        hex_masked.1.collect::<Vec<_>>()
    );

    Ok(())
}
