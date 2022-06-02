use {eyre::Result, git2::Repository, save::git2::CommitExt};
use bitvec::prelude::*;

// I could return bits + a mask


pub(crate) fn hex_bits(s: impl AsRef<str>) -> BitVec<u8> {
    let mut s = s.as_ref();

    // This is an upper-bound on the actual required capacity,
    // because it still includes spacing and other characters
    // that we will ignore below, but should still be a lot
    // better than requiring the Vec to grow dynamically.
    let mut bits = BitVec::<u8>::with_capacity(s.len() * 4);

    if s.starts_with("0x") || s.starts_with("0X") {
        s = &s[2..];
    }

    for c in s.chars() {
        match c {
            // ignore whitespace, underscores, and commas
            '_' | ' ' | '\t' | '\n' | '"' | '\'' | ',' => {},
            '0' => bits.extend([false, false, false, false]),
            '1' => bits.extend([false, false, false, true]),
            '2' => bits.extend([false, false, true, false]),
            '3' => bits.extend([false, false, true, true]),
            '4' => bits.extend([false, true, false, false]),
            '5' => bits.extend([false, true, false, true]),
            '6' => bits.extend([false, true, true, false]),
            '7' => bits.extend([false, true, true, true]),
            '8' => bits.extend([true, false, false, false]),
            '9' => bits.extend([true, false, false, true]),
            'a' | 'A' => bits.extend([true, false, true, false]),
            'b' | 'B' => bits.extend([true, false, true, true]),
            'c' | 'C' => bits.extend([true, true, false, false]),
            'd' | 'D' => bits.extend([true, true, false, true]),
            'e' | 'E' => bits.extend([true, true, true, false]),
            'f' | 'F' => bits.extend([true, true, true, true]),
            _ => panic!("invalid character {c:?} in hex string"),
        }
    }

    bits
}

pub(crate) fn hex_bytes(s: impl AsRef<str>) -> Vec<u8> {
    let bits = hex_bits(s.as_ref());
    let bytes = bits.as_raw_slice();
    dbg!(bytes);
    bytes.iter().copied().collect()
}

macro_rules! hex_bytes {
    [$($hex:tt)*] => {
        crate::hex_bytes(stringify!($($hex)*))
    }
}

macro_rules! hex_bits {
    [$($hex:tt)*] => {
        crate::hex_bits(stringify!($($hex)*))
    }
}

fn main() -> Result<()> {
    let repo = Repository::open_from_env()?;
    let head = repo.head()?.peel_to_commit()?;
    let tree = head.tree()?;
    let head = &head.id().to_string()[..8];
    let tree = &tree.id().to_string()[..4];

    let revision = 409;
    let generation = 1647;
    let number = 1862;
    println!("
               id: {head}
          message: {tree} at r{revision} / g{generation} / n{number}
    ");

    let some_bytes = hex_bytes![0x_FF_FF_F];
    println!("as damn bytes: {some_bytes:02x?}");
    let some_bits = hex_bits![0xFFFFF];
    println!("as bits: {some_bits:02x?}");



    Ok(())
}
