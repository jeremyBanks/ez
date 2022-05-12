#![allow(unused)]
use {
    crossterm::style::Stylize,
    git2::Oid,
    itertools::chain,
    std::{io::Write, path::PathBuf},
};

mod index_sources;

/// Git object identifier, a SHA-1 digest.
type GitId = [u8; 20];
/// Crate release hash, a SHA-256 digest.
type ReleaseHash = [u8; 32];

#[derive(Debug, Clone)]
pub struct State {
    validated_head: GitId,
    proposed_heads: Vec<GitId>,
}

#[ez::ly]
pub fn main() {
    let app_repo = home_dir().unwrap().tap_mut(|path| path.push(".cargo-merk/.git"));
    let app_repo = git2::Repository::init_opts(
        app_repo,
        git2::RepositoryInitOptions::new().bare(true).initial_head("trunk"),
    )?;

    for data_repo in index_sources::data_repos() {
        info!("Connecting to {data_repo}.");
        let mut data_repo = app_repo.remote_anonymous(&data_repo)?;
        data_repo.connect(git2::Direction::Fetch)?;

        let heads: OrderedMap<String, GitId> = data_repo
            .list()?
            .iter()
            .map(|head| (head.name().to_string(), head.oid().as_bytes().try_into().unwrap()))
            .collect();

        let head_names = heads.iter().map(|(name, _)| name.as_str()).collect_vec();

        info!("Downloading {head_names:?}.");

        data_repo.download(
            &head_names,
            Some(
                git2::FetchOptions::default()
                    .download_tags(git2::AutotagOption::None)
                    .prune(git2::FetchPrune::Off)
                    .update_fetchhead(false),
            ),
        )?;

        for id in heads.values() {
            debug!("Tagging {id:?}.");
            let hex_id = hex_encode(id);
            app_repo.tag_lightweight(
                &hex_id,
                &app_repo.find_object(Oid::from_bytes(&*id)?, Some(git2::ObjectType::Commit))?,
                true,
            )?;
        }
    }

    // for repo in index_sources::head_repos() {
    //     let heads = discover_refs(&repo)?;
    //     let head =
    // heads.get("HEAD").map(hex_encode).unwrap_or("<None??>".to_string());

    //     println!("{repo}\n  is at {head}\n");

    //     println!("\n\n");
    // }

    // let project_manifest: Toml =
    // std::fs::read_to_string("Cargo.toml")?.parse()?;
    // let project_lockfile: Toml =
    // std::fs::read_to_string("Cargo.lock")?.parse()?;

    // Step 1: fetch all index mirrors
    //         If this is our first fetch, verify that the only root commit
    //         is the expected commit, a33de1c98898dc1baf541ee2c5162e7baea7c838,
    //         and that it includes some recent-as-of-time-of-publication
    //         commit, such as d3309ab55b6adc1151b1d1004ff23e9240d55279.
    //
    // Step 2: verify index/mirrors are consistent with each other and with
    //         our previous local head, and that none of the intermediate
    // commits         violate any constraints:
    //          (a) single parent, or no parents if squashed, but squashes
    // should still              include a reference to the effective parent
    // in the messages.          (b) no checksums should be changed.
    //          (c) no versions should be removed
    //              (this probably needs to allow some exceptions, as long as no
    // new               versions are added with conflicting checksums.)
    //
    //         The local state we keep will probably be a sled-or-similar
    // database         with all { (name, version) -> checksum } pairs and
    // our latest head         commit ID. If possible, we could then
    // truncate all git history         before the HEAD.
    //
    // Step 3: drop any git history we don't need
    //
    // Step 4: read Cargo.lock file to identify all dependencies of current
    //         project, or quit if no current project.
    //
    // Step 5: for each dependency, find its files and...
    //
    // Step 6: verify their checksum (even though cargo already should have)
    //
    // Step 7: if it has a repository in its Cargo.toml, fetch that.
    //
    // Step 8: if it has a commit tagged in the repository, find that commit,
    //         otherwise log a warning but try to find a commit whose tag is
    //         the version number, with or without a prefix v, maybe.
    //
    // Step 9: if we have a release commit, check for any files in the released
    //         bundle which do not match those in the git repository. The
    // obvious         exception is the normalization of Cargo.toml, but we
    // should at         least verify that they are semantically equivalent
    // except for         the expected variations.
    //
    // Step 10: If the cargo bundle has any duplicate files, flag as malicious.

    // Brainstorming:
    // - Maybe we should have a way to record the index version that was
}

/// Returns all of the refs advertised by a Git repository.
#[throws]
fn discover_refs(git_url: &str) -> OrderedMap<String, GitId> {
    let mut remote = git2::Remote::create_detached(git_url)?;
    remote.connect(git2::Direction::Fetch)?;
    remote
        .list()?
        .iter()
        .map(|head| (head.name().to_string(), head.oid().as_bytes().try_into().unwrap()))
        .collect()
}

/// Returns a formatted ref advertisement as would be returned by a Git
/// repository in response to an `info/refs?service=git-upload-pack` discovery
/// request.
#[throws]
fn announce_refs(mut refs: OrderedMap<String, GitId>) -> Vec<u8> {
    let mut buf = Vec::with_capacity(512);
    let mut lines: Vec<Vec<u8>> = Vec::with_capacity(8);

    lines.push(b"# service=git-upload-pack\n".to_vec());
    lines.push(vec![]);

    let canonical_head = refs.remove("HEAD").into_iter().map(|head| ("HEAD".to_string(), head));

    for (i, head) in chain(canonical_head, refs.into_iter().sorted()).enumerate() {
        let (name, git_id) = head;
        let hex_id = hex_encode(git_id);

        let mut line = format!("{name} {hex_id}");
        if i == 0 {
            line.push('\0');
        }
        line.push('\n');

        lines.push(line.as_bytes().to_vec());
    }

    lines.push(vec![]);

    for line in lines {
        if line.is_empty() {
            buf.extend(b"0000");
        } else {
            buf.extend(format!("{:04x}", line.len()).as_bytes().to_vec());
            buf.extend(line.to_vec());
        }
    }

    buf
}