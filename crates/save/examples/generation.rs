use {eyre::Result, git2::Repository, save::git2::CommitExt};

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

    Ok(())
}
