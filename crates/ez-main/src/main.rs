use {
    ez_core as ez,
    std::borrow::Cow,
    tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt},
};

pub trait ExitStatus {
    fn to_i32(&self) -> i32;
}

impl ExitStatus for u8 {
    fn to_i32(&self) -> i32 {
        i32::from(*self)
    }
}

impl ExitStatus for i32 {
    fn to_i32(&self) -> i32 {
        *self
    }
}

impl ExitStatus for () {
    fn to_i32(&self) -> i32 {
        0
    }
}

pub fn entry_point<
    Args: FromIterator<String>,
    Env: FromIterator<(String, String)>,
    Return: ExitStatus,
>(
    main_package_name: &str,
    main: fn(Args, Env) -> Result<Return, ez::Error>,
) -> Result<(), ez::Error> {
    // SAFETY: Modifying environment variables can be risky business in the
    // presence of other threads. We're relying on the fact that this is the
    // entry point and no other threads should exist yet, and then pass a
    // safely-frozen copy of the environment to the main function.

    dotenv::dotenv().ok();

    if std::env::var("RUST_LOG").unwrap_or_default().is_empty() {
        if cfg!(debug_assertions) {
            std::env::set_var("RUST_LOG", format!("warn,{main_package_name}=debug,ez=debug"));
        } else {
            std::env::set_var("RUST_LOG", format!("warn,{main_package_name}=info,ez=info"));
        }
    }

    if std::env::var("RUST_SPANTRACE").unwrap_or_default().is_empty() {
        std::env::set_var("RUST_SPANTRACE", "1");
    }

    color_eyre::install().unwrap();

    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_tree::HierarchicalLayer::new(2).with_indent_lines(true))
        .init();

    let args = std::env::args_os().skip(1).map(|s| match s.to_string_lossy() {
        Cow::Borrowed(lossless) => lossless.to_owned(),
        Cow::Owned(lossy) => {
            tracing::warn!(
                target: "ez",
                "Invalid UTF-8 in command-line argument. Invalid sequences have been replaced \
                 with '�':\n  {lossy:?}"
            );
            lossy
        }
    });

    let env = std::env::vars_os().filter_map(|(name, value)| {
        let name = name
            .to_str()
            .or_else(|| {
                let lossy = name.to_string_lossy();
                tracing::warn!(
                    target: "ez",
                    "Invalid UTF-8 in an environment variable name ({lossy:?}). It has been \
                     skipped."
                );
                None
            })?
            .to_owned();
        let value = value
            .to_str()
            .or_else(|| {
                tracing::warn!(
                    target: "ez",
                    "Invalid UTF-8 in the value of the environment variable {name:?}. It has been \
                     skipped."
                );
                None
            })?
            .to_owned();
        Some((name, value))
    });

    let exit_status = main(args.collect(), env.collect()).map_err(|err| {
        tracing::error!(target: "ez", "exiting with error status code due to an unhandled error");
        err
    })?.to_i32();

    if exit_status != 0 {
        tracing::debug!(target: "ez", "exiting with error status code {}", exit_status);
        std::process::exit(exit_status);
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
/// Drops an iterator without consuming any elements.
pub struct IteratorDropper;
impl<Item> FromIterator<Item> for IteratorDropper {
    fn from_iter<Iterator: IntoIterator<Item = Item>>(_: Iterator) -> Self {
        IteratorDropper
    }
}
