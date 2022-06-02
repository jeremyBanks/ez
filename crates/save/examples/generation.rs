use {eyre::Result, git2::Repository, save::git2::CommitExt};

fn main() -> Result<()> {
    let repo = Repository::open_from_env()?;
    let head = repo.head()?.peel_to_commit()?;
    let tree = head.tree()?;
    let head = &head.id().to_string()[..4];
    let tree = &tree.id().to_string()[..4];

    let first_parent_depth = 409;
    let generation_index = 1647;
    let commit_index = 1862;

    println!("
        HEAD is at commit {head} containing tree {tree}

        r{first_parent_depth} / g{generation_index} / n{commit_index} / t{tree}
    ");

    Ok(())
}
