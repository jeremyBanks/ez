use {eyre::Result, git2::Repository, save::git2::CommitExt};
use bitvec::prelude::*;

pub(crate) fn hex_literal(literal: &str) -> BitVec<u8> {
    let string: String = literal
            .chars()
            .flat_map(|c| match c {
                '_' | ' ' | '\t' | '\n' | '"' | '\'' | ',' => None,
                '0'..='9' | 'a'..='f' | 'A'..='F' => Some(c),
                _ => panic!("Invalid character {c:?} in hex literal."),
            })
            .collect();
let bytes = Vec::<u8>::with_capacity((string.len() + 1) / 2);
    let bits = bytes.view_bits::<Lsb0>().to_bitvec();
    bits
}

macro_rules! hex {
    [$($(0)$(x)+)? $($hex:tt)*] => {
        crate::hex_literal(stringify!($($hex)*))
    }
}

fn main() -> Result<()> {
    let repo = Repository::open_from_env()?;
    let head = repo.head()?.peel_to_commit()?;
    let tree = head.tree()?;
    let head = &head.id().to_string()[..8];
    let tree = &tree.id().to_string()[..4];

    // let prefix = bits![u8; true, false, false, true];

    let revision = 409;
    let generation = 1647;
    let number = 1862;

    let empty_tree = hex![0x4b825dc642cb6eb9a060e54bf8d69288fbee4904];

    println!("
                    {empty_tree}
               id: {head}
          message: {tree} at r{revision} / g{generation} / n{number}
    ");

    Ok(())
}
