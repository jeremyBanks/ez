use save::{
    git2::CommitExt,
    hex::{hex, hex_masked},
};

fn main() -> ::eyre::Result<()> {
    let repo = ::git2::Repository::open_from_env()?;
    let head = repo.head()?.peel_to_commit()?;
    let tree = head.tree()?;
    let head_str = &head.id().to_string()[..8];
    let tree_str = &tree.id().to_string()[..8];

    // revision of initial commit is 0
    // revision of any other commit is its first parent + 1
    let revision = 204;
    // generation of initial commit is 0
    // generation of any other commit is the maximum of its parents generations + 1
    let generation = head.generation_number();
    // number of the initial commit is 0
    // number of any commit is the total number of commits in its ancestry graph - 1
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
        hex_masked.0, hex_masked.1
    );

    let hex_masked = hex_masked![4b825dc642cb6eb9a060e54bf8d69288fbee49045];
    println!(
        "hex_masked:
        {:02x?}
        {:02x?}
    ",
        hex_masked.0, hex_masked.1
    );

    Ok(())
}
