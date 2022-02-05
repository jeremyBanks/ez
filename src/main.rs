//! This module provides a `#[main]` macro intended for use on your
//! entry-point `main` function.

use std::borrow::Cow;

/// `#[ez::main]` macro for use on your entry-point `main` function.
pub use ez_proc_macros::main;

#[doc(hidden)]
pub fn run<Args: FromIterator<String>, Env: FromIterator<(String, String)>>(
    main_package_name: &str,
    main: fn(Args, Env) -> Result<(), eyre::Report>,
) -> Result<(), eyre::Report> {
    dotenv::dotenv().ok();

    // SAFETY: Modifying environment variables can be risky business in the
    // presence of other threads. We're relying on the fact that this is the
    // entry point and no other threads should exist yet, and then pass a
    // safely-frozen copy of the environment to the main function.

    if std::env::var("RUST_LOG").unwrap_or_default().is_empty() {
        if cfg!(debug_assertions) {
            std::env::set_var("RUST_LOG", format!("warn,{main_package_name}=debug"));
        } else {
            std::env::set_var("RUST_LOG", format!("warn,{main_package_name}=info"));
        }
    }

    if std::env::var("RUST_SPANTRACE")
        .unwrap_or_default()
        .is_empty()
    {
        std::env::set_var("RUST_SPANTRACE", "1");
    }

    color_eyre::install().unwrap();

    tracing_subscriber::util::SubscriberInitExt::init(tracing_subscriber::Layer::with_subscriber(
        tracing_error::ErrorLayer::default(),
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_target(true)
            .with_span_events(
                tracing_subscriber::fmt::format::FmtSpan::NEW
                    | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            )
            .finish(),
    ));

    let args = std::env::args_os()
        .skip(1)
        .map(|s| match s.to_string_lossy() {
            Cow::Borrowed(lossless) => lossless.to_owned(),
            Cow::Owned(lossy) => {
                tracing::warn!(
                    "Invalid UTF-8 in command-line argument. Invalid sequences have been replaced \
                     with '�':\n  {:?}",
                    lossy
                );
                lossy
            },
        });

    let env = std::env::vars_os().filter_map(|(name, value)| {
        let name = name
            .to_str()
            .or_else(|| {
                let lossy = name.to_string_lossy();
                tracing::warn!(
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
                    "Invalid UTF-8 in the value of the environment variable {name:?}. It has been \
                     skipped."
                );
                None
            })?
            .to_owned();
        Some((name, value))
    });

    main(args.collect(), env.collect())
}
