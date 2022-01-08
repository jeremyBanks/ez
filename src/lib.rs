#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc
)]

use {
    git2::{Commit, Oid},
    std::{cell::RefCell, cmp::max, collections::HashMap, rc::Rc},
    thousands::Separable,
    tracing::{debug_span, instrument},
    ::{
        clap::Parser,
        digest::Digest,
        eyre::{bail, Result, WrapErr},
        git2::{ErrorCode, Repository, RepositoryInitOptions, Signature, Time},
        rayon::prelude::*,
        std::{env, fs},
        tracing::{debug, info, trace, warn},
    },
};

/// Would you like to SAVE the change?
///
/// Commit everything in the current Git repository, no questions asked.
#[derive(Parser, Debug, Clone)]
#[clap(version)]
#[remain::sorted]
pub struct Args {
    /// Prepare the commit, but don't actually save anything to disk.
    #[clap(long)]
    pub dry_run: bool,

    /// The author email to use for the commit.
    ///
    /// [default: email from git, or else from parent commit, or else "save"]
    #[clap(long, short = 'e')]
    pub email: Option<String>,

    /// The target commit hash or prefix, in hex.
    ///
    /// [default: the commit's tree hash]
    #[clap(long = "hash", short = 'x')]
    pub hash_hex: Option<String>,

    /// Use a manual commit message instead of the default generated message.
    #[clap(long, short = 'm')]
    pub message: Option<String>,

    /// The author name to use for the commit.
    ///
    /// [default: name from git, or else from parent commit, or else "save"]
    #[clap(long, short = 'n')]
    pub name: Option<String>,

    /// The time is NOW.
    ///
    /// [default: the time is ACTUALLY now]
    #[clap(long = "now", short = 'w')]
    pub now_seconds: Option<i64>,

    /// Decrease log verbosity. May be used multiple times.
    #[clap(long, short = 'q', parse(from_occurrences))]
    pub quiet: i32,

    /// Seconds of timestamp allocated for each commit to search.
    #[clap(long="step", short='s', default_value_t = 64 * 2)]
    pub step_seconds: u32,

    /// Increase log verbosity. May be used multiple times.
    #[clap(long, short = 'v', parse(from_occurrences))]
    pub verbose: i32,

    /// Proceed in spite of any warnings.
    #[clap(long, short = 'y')]
    pub yes: bool,
}

/// CLI entry point.
///
/// # Panics
///
/// For some fatal errors.
///
/// # Errors
///
/// For other fatal errors.
#[instrument(level = "debug")]
pub fn main(args: Args) -> Result<()> {
    let repo = match Repository::open_from_env() {
        Ok(repo) => {
            if repo.is_bare() {
                bail!(
                    "Found Git repository, but it was bare (no working directory): {:?}",
                    repo.path()
                );
            }

            debug!("Found Git repository: {:?}", repo.workdir().unwrap());
            repo
        },
        Err(_err) => {
            let path = std::env::current_dir()?;
            let empty = fs::read_dir(&path)?.next().is_none();
            info!("No Git repository found.");

            let dangerous = (path == home::home_dir().unwrap()) || (path.to_str() == Some("/"));

            if dangerous && !args.yes {
                bail!(
                    "Current directory seems important, skipping auto-init (-y/--yes to override)."
                );
            } else if empty && !args.yes {
                bail!("Current directory is empty, skipping auto-init (-y/--yes to override).");
            } else {
                info!("Initializing a new Git repository in: {:?}", path);
                Repository::init_opts(
                    path,
                    RepositoryInitOptions::new()
                        .initial_head("trunk")
                        .no_reinit(true),
                )?
            }
        },
    };

    let head = match repo.head() {
        Ok(head) => Some(head.peel_to_commit().unwrap()),
        Err(err) if err.code() == ErrorCode::UnbornBranch => None,
        Err(err) => {
            bail!("Unexpected error from Git: {:#?}", err);
        },
    };

    let config = repo.config()?;

    let user_name: String = {
        if let Some(args_name) = args.name {
            trace!(
                "Using author name from command line argument: {:?}",
                &args_name
            );
            args_name
        } else if let Ok(config_name) = config.get_string("user.name") {
            debug!(
                "Using author name from Git configuration: {:?}",
                &config_name
            );
            config_name
        } else if let Some(previous_name) = head
            .as_ref()
            .and_then(|x| x.author().name().map(|x| x.to_string()))
        {
            info!(
                "Using author name from previous commit: {:?}",
                &previous_name
            );
            previous_name
        } else {
            let placeholder_name = "save";
            warn!(
                "No author name found, falling back to placeholder: {:?}",
                &placeholder_name
            );
            placeholder_name.to_string()
        }
    };

    let user_email: String = if let Some(args_email) = args.email {
        trace!(
            "Using author email from command line argument: {:?}",
            &args_email
        );
        args_email
    } else if let Ok(config_email) = config.get_string("user.email") {
        debug!(
            "Using author email from Git configuration: {:?}",
            &config_email
        );
        config_email
    } else if let Some(previous_email) = head
        .as_ref()
        .and_then(|x| x.author().email().map(|x| x.to_string()))
    {
        info!(
            "Using author email from previous commit: {:?}",
            &previous_email
        );
        previous_email
    } else {
        let placeholder_email = "save";
        warn!(
            "No author email found, falling back to placeholder: {:?}",
            &placeholder_email
        );
        placeholder_email.to_string()
    };

    let generation_index = head
        .as_ref()
        .map(|commit| find_generation_index(commit) + 1)
        .unwrap_or(0);

    let mut index = repo.index()?;

    index.add_all(["*"], Default::default(), Default::default())?;
    let tree = index.write_tree()?;

    if let Some(ref head) = head {
        if tree == head.tree_id() {
            if args.message.is_some() {
                info!("Committing with only a message.");
            } else if args.yes {
                info!("Committing with no changes.");
            } else {
                info!("Nothing to commit (-y/--yes to override).");
                return Ok(());
            }
        }
    }

    if !args.dry_run {
        index.write()?;
    } else {
        info!("Skipping index write because this is a dry run.");
    }

    let tree4 = &tree.to_string()[..4];
    let tree = repo.find_tree(tree)?;

    let revision_index = generation_index + 1;
    let message = args.message.unwrap_or_else(|| {
        let mut message = format!("r{}", revision_index);
        if let Some(ref head) = head {
            message += &format!("/{}/{}", tree4, &head.id().to_string()[..4]);
        } else if tree.iter().next().is_some() {
            message += &format!("/{}", &tree4);
        }
        message
    });

    let previous_seconds = head.as_ref().map(|c| c.time().seconds()).unwrap_or(0);
    let time = Signature::now(&user_name, &user_email)?.when();
    let mut seconds = args.now_seconds.unwrap_or_else(|| time.seconds());
    let offset = 0;

    let seconds_since_head = seconds - previous_seconds;

    let step_seconds = i64::from(args.step_seconds);
    let snap_seconds = step_seconds * 2;
    let slack_seconds = step_seconds * 4;

    if seconds_since_head < slack_seconds {
        seconds = previous_seconds + step_seconds;
    } else {
        seconds = seconds - seconds % snap_seconds
    }

    let parents = &head.iter().collect::<Vec<_>>();

    let min_timestamp = seconds;
    let max_timestamp = seconds + step_seconds - 1;

    let mut target_hash = args
        .hash_hex
        .map(|s| hex::decode(s).wrap_err("target hash must be hex").unwrap())
        .unwrap_or_default();
    target_hash.append(&mut tree.id().as_bytes().to_vec());

    let base_commit = repo
        .commit_create_buffer(
            &Signature::new(&user_name, &user_email, &Time::new(min_timestamp, offset)).unwrap(),
            &Signature::new(&user_name, &user_email, &Time::new(min_timestamp, offset)).unwrap(),
            &message,
            &tree,
            parents,
        )
        .unwrap()
        .to_vec();
    let base_commit = std::str::from_utf8(&base_commit)
        .wrap_err("commit must be valid utf-8")
        .unwrap();

    let (author_timestamp, commit_timestamp, brute_hash, commit_buffer) =
        brute_commit(base_commit, &target_hash, min_timestamp, max_timestamp);

    if !args.dry_run {
        repo.commit(
            Some("HEAD"),
            &Signature::new(
                &user_name,
                &user_email,
                &Time::new(author_timestamp, offset),
            )
            .unwrap(),
            &Signature::new(
                &user_name,
                &user_email,
                &Time::new(commit_timestamp, offset),
            )
            .unwrap(),
            &message,
            &tree,
            parents,
        )?;
    } else {
        info!(
            "Skipping commit write because this is a dry run:\n\ncommit {}\n{}",
            brute_hash
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(""),
            commit_buffer
        );
    }

    // eprintln!();

    // Command::new("git")
    //     .args(&[
    //         "--no-pager",
    //         "log",
    //         "--name-status",
    //         "--format=raw",
    //         "--graph",
    //         "--decorate",
    //         "-n",
    //         "2",
    //     ])
    //     .status()?;

    // eprintln!();

    Ok(())
}

/// Given a raw Git commit as a string, find the timestamps that will produce
/// the closest commit ID to target_hash.
#[instrument(level = "debug")]
pub fn brute_commit(
    base_commit: &str,
    target_hash: &[u8],
    min_timestamp: i64,
    max_timestamp: i64,
) -> (i64, i64, Vec<u8>, String) {
    let base_commit_lines = base_commit.split('\n').collect::<Vec<&str>>();
    let author_line_index = base_commit_lines
        .iter()
        .position(|line| line.starts_with("author "))
        .unwrap();
    let author_line_pieces = &base_commit_lines[author_line_index]
        .split(' ')
        .collect::<Vec<_>>();
    let committer_line_index = base_commit_lines
        .iter()
        .position(|line| line.starts_with("committer "))
        .unwrap();
    let committer_line_pieces = &base_commit_lines[committer_line_index]
        .split(' ')
        .collect::<Vec<_>>();

    let commit_create_buffer = |author_timestamp: i64, committer_timestamp: i64| {
        let mut commit_lines = base_commit_lines.clone();

        let mut author_line_pieces = author_line_pieces.clone();
        let i = author_line_pieces.len() - 2;
        let author_timestamp = author_timestamp.to_string();
        author_line_pieces[i] = &author_timestamp;
        let author_line = author_line_pieces.join(" ");
        commit_lines[author_line_index] = &author_line;

        let mut committer_line_pieces = committer_line_pieces.clone();
        let i = committer_line_pieces.len() - 2;
        let committer_timestamp = committer_timestamp.to_string();
        committer_line_pieces[i] = &committer_timestamp;
        let committer_line = committer_line_pieces.join(" ");
        commit_lines[committer_line_index] = &committer_line;

        commit_lines.join("\n")
    };

    let (_score, author_timestamp, commit_timestamp, hash, candidate) = ((min_timestamp
        ..=max_timestamp)
        .into_par_iter()
        .map(|author_timestamp| {
            (author_timestamp..=max_timestamp)
                .into_par_iter()
                .map(|commit_timestamp| {
                    let candidate = commit_create_buffer(author_timestamp, commit_timestamp);
                    let hash = sha1::Sha1::new()
                        .chain_update(format!("commit {}", candidate.len()))
                        .chain_update([0x00])
                        .chain_update(&candidate)
                        .finalize()
                        .to_vec();

                    let score = hash
                        .iter()
                        .zip(target_hash.iter())
                        .map(|(a, b)| (a ^ b))
                        .collect::<Vec<u8>>();

                    (score, author_timestamp, commit_timestamp, hash, candidate)
                })
                .min()
                .unwrap()
        }))
    .min()
    .unwrap();

    (author_timestamp, commit_timestamp, hash, candidate)
}

/// Finds the generation index of a given Git commit.
///
/// The generation index is the number of edges of the longest path between the
/// given commit and an initial commit (one with no parents, which has an
/// implicit generation index of 0).
#[instrument(level = "debug")]
pub fn find_generation_index(commit: &Commit) -> u32 {
    let head = commit.clone();

    #[derive(Debug, Clone)]
    struct CommitNode {
        // number of edges (git children) whose distances hasn't been accounted-for yet
        unaccounted_edges_in: u32,
        // max distance from head of accounted-for edges
        max_distance_from_head: u32,
        // git parents of this node
        edges_out: Vec<Rc<RefCell<CommitNode>>>,
    }

    let (root, _leaves) = {
        let span = debug_span!("load_git_graph");
        let _guard = span.enter();

        let mut all_commits = HashMap::<Oid, Rc<RefCell<CommitNode>>>::new();
        let mut initial_commits = vec![];

        #[derive(Debug, Clone)]
        struct CommitWalking<'repo> {
            commit: Commit<'repo>,
            from: Option<Rc<RefCell<CommitNode>>>,
        }

        let mut walks = vec![CommitWalking {
            commit: head.clone(),
            from: None,
        }];

        while let Some(CommitWalking { commit, from }) = walks.pop() {
            let from = &from;
            all_commits
                .entry(commit.id())
                .and_modify(|node| {
                    if let Some(from) = from {
                        from.borrow_mut().edges_out.push(node.clone());
                        node.borrow_mut().unaccounted_edges_in += 1;
                    }
                })
                .or_insert_with(|| {
                    let node = Rc::new(RefCell::new(CommitNode {
                        edges_out: vec![],
                        unaccounted_edges_in: 1,
                        max_distance_from_head: 0,
                    }));

                    if let Some(from) = from {
                        from.borrow_mut().edges_out.push(node.clone());
                    }

                    if commit.parents().len() == 0 {
                        debug!("Found an initial commit: {:?}", commit);
                        initial_commits.push(node.clone());
                    } else {
                        for parent in commit.parents() {
                            walks.push(CommitWalking {
                                commit: parent,
                                from: Some(node.clone()),
                            });
                        }
                    }

                    node
                });
        }

        info!(
            "Loaded {} commits, containing {} initial commits.",
            all_commits.len().separate_with_underscores(),
            initial_commits.len(),
        );

        let head = all_commits.get(&head.id()).unwrap().clone();
        (head, initial_commits)
    };

    let generation_index = {
        let span = debug_span!("measure_git_graph");
        let _guard = span.enter();

        let mut generation_index = 0;

        let mut live = vec![root];

        while let Some(commit) = live.pop() {
            let commit = commit.borrow_mut();

            if commit.edges_out.is_empty() {
                generation_index = max(generation_index, commit.max_distance_from_head);
            } else {
                for parent in commit.edges_out.iter() {
                    let mut parent_mut = parent.borrow_mut();
                    parent_mut.max_distance_from_head = max(
                        parent_mut.max_distance_from_head,
                        commit.max_distance_from_head + 1,
                    );
                    parent_mut.unaccounted_edges_in -= 1;

                    if parent_mut.unaccounted_edges_in == 0 {
                        live.push(parent.clone());
                    }
                }
            }
        }

        generation_index
    };

    generation_index
}

/// Initialize the typical global environment and parses the typical [Args] for
/// save's [main] CLI entry point.
///
/// # Panics
///
/// This will panic if called multiple times, or if other code attempts
/// conflicting global initialization of systems such as logging.
pub fn init() -> Args {
    color_eyre::install().unwrap();

    let args = Args::parse();

    let default_verbosity = 3;

    let log_env = env::var("RUST_LOG").unwrap_or_default();

    let log_level = if args.verbose == 0 && args.quiet == 0 && !log_env.is_empty() {
        log_env
    } else {
        match default_verbosity + args.verbose - args.quiet {
            i32::MIN..=0 => "off".into(),
            1 => "error".into(),
            2 => "warn".into(),
            3 => "info".into(),
            4 => "debug".into(),
            5..=i32::MAX => "trace".into(),
        }
    };

    tracing_subscriber::util::SubscriberInitExt::init(tracing_subscriber::Layer::with_subscriber(
        tracing_error::ErrorLayer::default(),
        tracing_subscriber::fmt()
            .with_env_filter(::tracing_subscriber::EnvFilter::new(log_level))
            .with_target(false)
            .with_span_events(
                tracing_subscriber::fmt::format::FmtSpan::ENTER
                    | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            )
            .compact()
            .finish(),
    ));

    args
}
