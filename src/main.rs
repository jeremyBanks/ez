//! This module provides a `#[main]` macro, intended for use on your
//! entry-point `main` function.

pub use ez_internal::main;

#[doc(hidden)]
pub fn pre_main() {
    panic!("This is a test");
}

#[doc(hidden)]
pub fn call_main<Args, Env, Return>(f: fn(Args, Env) -> Return)
where
    Args: FromIterator<String>,
    Env: FromIterator<(String, String)>,
    Return: MainResult,
{
    todo!()
}

fn test_a() -> impl MainResult {

}

fn test_b() -> impl MainResult {
    Ok({
        if 1 % 2 == 3 {
            Err(eyre::eyre!("This is a test"))?;
        }
        return;
    })
}

#[doc(hidden)]
/// We allow main functions to return `()`, `u8`, or `u16`, optionally wrapped
/// in an `eyre::Result`.
pub trait MainResult {
    fn into(self) -> Result<Option<i32>, eyre::Report>;
}

impl MainResult for Result<(), eyre::Report> {
    fn into(self) -> Result<Option<i32>, eyre::Report> {
        Ok(None)
    }
}

impl MainResult for () {
    fn into(self) -> Result<Option<i32>, eyre::Report> {
        Ok(None)
    }
}

impl MainResult for i32 {
    fn into(self) -> Result<Option<i32>, eyre::Report> {
        Ok(Some(self))
    }
}

impl MainResult for Result<i32, eyre::Report> {
    fn into(self) -> Result<Option<i32>, eyre::Report> {
        self.map(Some)
    }
}


pub struct RetiredCollector;
impl<T> FromIterator<T> for RetiredCollector {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        drop(iter);
        RetiredCollector
    }
}

/*

    //! This module provides a `#[main]` macro, intended for use on your
    //! entry-point `main` function.

    use std::borrow::Cow;

    /// A `#[main]` macro adding reasonable defaults for a `main` entry point
    /// function.
    ///
    /// - enables [`::color-eyre`] for nicely-formatted colored error
    ///   backtraces.
    ///   - we default to `RUST_LIB_BACKTRACE=1` in `debug` builds
    /// - enables console output for log messages from `tracing` and `log`.
    ///   - we default to log level `WARN` for imported crates
    ///   - we default to log level `INFO` for the current crate
    /// - wraps main in a `Result` with an `eyre::Report` to enable use of `?`
    ///   to propagate top-level errors into a crash.
    /// - `main()` can optionally be defined with primitive integer return type.
    ///   if so, the process will use this value as its exit status, as in C.
    ///   (This ensures that all destructors on the stack have had a chance to
    ///   run, which is not the case when calling `std::process::exit()`
    ///   directly.)
    /// - `main()` can optionally be defined to take one or two arguments:
    ///   - `args`
    ///   - `env`
    /// THIS DOESn"T NEED TO BE DYNAMIC YOU IDIOT.
    ///
    #[doc(inline)]
    pub use ez_internal::main;

    #[doc(hidden)]
    /// Implements the runtime behaviour [`#[main]`] wraps around the inner
    /// function.
    pub fn run_main<F: Main>(
        f: F,
    ) -> ::core::result::Result<core::convert::Infallible, ::eyre::Report> {
        // Load environment variables from the nearest `.env` file, if one exists.
        dotenv::dotenv().ok();

        if std::env::var("RUST_LOG").unwrap_or_default().is_empty() {
            std::env::set_var("RUST_LOG", "trace");
        }

        if std::env::var("RUST_BACKTRACE")
            .unwrap_or_default()
            .is_empty()
        {
            if cfg!(debug_assertions) {
                std::env::set_var("RUST_BACKTRACE", "1");
            } else {
                std::env::set_var("RUST_BACKTRACE", "0");
            }
        }

        if std::env::var("RUST_SPANTRACE")
            .unwrap_or_default()
            .is_empty()
        {
            std::env::set_var("RUST_SPANTRACE", "1");
        }

        // Add fancier backtraces to errors.
        color_eyre::install().unwrap();

        tracing_subscriber::util::SubscriberInitExt::init(
            tracing_subscriber::Layer::with_subscriber(
                tracing_error::ErrorLayer::default(),
                tracing_subscriber::fmt()
                    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                    .with_target(false)
                    .with_span_events(
                        tracing_subscriber::fmt::format::FmtSpan::NEW
                            | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
                    )
                    .finish(),
            ),
        );

        let args = std::env::args_os()
            .skip(1)
            .map(|s| match s.to_string_lossy() {
                Cow::Borrowed(lossless) => lossless.to_owned(),
                Cow::Owned(lossy) => {
                    tracing::warn!(
                        "Invalid UTF-8 in command-line argument. Invalid sequences have been \
                         replaced with '�':\n  {:?}",
                        lossy
                    );
                    lossy
                },
            });
        let args = F::Args::from_iter(args);

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
                        "Invalid UTF-8 in the value of the environment variable {name:?}. It has \
                         been skipped."
                    );
                    None
                })?
                .to_owned();
            Some((name, value))
        });
        let env = F::Env::from_iter(env);

        let status = f.call(args, env)?.into_i32();

        std::process::exit(status)
    }

    #[doc(hidden)]
    /// A trait covering supported types for the function wrapped by
    /// [`#[main]`].
    pub trait Main {
        type Args: MainArgs;
        type Env: MainEnv;
        type ExitStatus: MainExitStatus;

        fn call(&self, args: Self::Args, env: Self::Env) -> Result<Self::ExitStatus, eyre::Report>;
    }

    impl<Args, Env, ExitStatus> Main for fn(Args, Env) -> Result<ExitStatus, eyre::Report>
    where
        Args: MainArgs,
        Env: MainEnv,
        ExitStatus: MainExitStatus,
    {
        type Args = Args;
        type Env = Env;
        type ExitStatus = ExitStatus;

        fn call(&self, args: Self::Args, env: Self::Env) -> Result<Self::ExitStatus, eyre::Report> {
            self(args, env)
        }
    }

    impl<Args, ExitStatus> Main for fn(Args) -> Result<ExitStatus, eyre::Report>
    where
        Args: MainArgs,
        ExitStatus: MainExitStatus,
    {
        type Args = Args;
        type Env = CollectNothing;
        type ExitStatus = ExitStatus;

        fn call(&self, args: Self::Args, _: Self::Env) -> Result<Self::ExitStatus, eyre::Report> {
            self(args)
        }
    }

    impl<ExitStatus> Main for fn() -> Result<ExitStatus, eyre::Report>
    where
        ExitStatus: MainExitStatus,
    {
        type Args = CollectNothing;
        type Env = CollectNothing;
        type ExitStatus = ExitStatus;

        fn call(&self, _: Self::Args, _: Self::Env) -> Result<Self::ExitStatus, eyre::Report> {
            self()
        }
    }

    pub struct CollectNothing;
    impl<T> FromIterator<T> for CollectNothing {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            drop(iter);
            CollectNothing
        }
    }

    pub trait MainEnv: FromIterator<(String, String)> {}
    impl<T> MainEnv for T where T: FromIterator<(String, String)> {}

    pub trait MainArgs: FromIterator<String> {}
    impl<T> MainArgs for T where T: FromIterator<String> {}

    pub trait MainExitStatus {
        fn into_i32(self) -> i32;
    }

    impl MainExitStatus for i32 {
        fn into_i32(self) -> i32 {
            self
        }
    }

    impl MainExitStatus for u8 {
        fn into_i32(self) -> i32 {
            self.into()
        }
    }

    impl MainExitStatus for i8 {
        fn into_i32(self) -> i32 {
            self.into()
        }
    }

    impl MainExitStatus for u16 {
        fn into_i32(self) -> i32 {
            self.into()
        }
    }

    impl MainExitStatus for i16 {
        fn into_i32(self) -> i32 {
            self.into()
        }
    }

    impl MainExitStatus for () {
        fn into_i32(self) -> i32 {
            0
        }
    }
}

pub mod errors {
    //! This module provides function macros to simplify error propagation.

    /// Returns from the enclosing function with a [`Result::Err`].
    ///
    /// This will contain either a provided error object, or an error message
    /// (if the first argument is a formatting string literal). Error messages
    /// are formatted into [`eyre::Report`]s using [`eyre::eyre!`], so you'll
    /// get an error if you try to use an error message in a function that
    /// expects a more-specific error type in its [`Result`].
    #[macro_export]
    macro_rules! throw {
        ($msg:literal $(, $rest:tt)* $(,)?) => {
            return ::core::result::Result::Err(::eyre::eyre!($msg $(, $rest)*));
        };

        ($error:expr) => {
            return ::core::result::Result::Err(core::convert::From::from($error));
        };
    }

    #[doc(inline)]
    /// Applied to the function to allow use of `?` operator in the body, but
    /// any errors will panic (crash) the program.
    pub use ez_internal::panics;
    #[doc(inline)]
    /// Applied to a function to return a [`Result`], with an [`eyre::Report`]
    /// by default.
    pub use ez_internal::throws;
    #[doc(inline)]
    /// Applied to a function to create two versions, one that panics on error
    /// (like [`panics`]), and a copy with `try_` prefixed to the name which
    /// returns a [`Result`] instead (like [`throws`]).
    pub use ez_internal::try_or_panics;
    #[doc(inline)]
    pub use throw;

    #[doc(hidden)]
    pub trait IntoResult<T, E> {
        fn into_result(self) -> Result<T, E>;
    }

    impl<T, E> IntoResult<T, E> for Result<T, E> {
        fn into_result(self) -> Result<T, E> {
            self
        }
    }

    impl<T, E> IntoResult<T, E> for T {
        fn into_result(self) -> Result<T, E> {
            Ok(self)
        }
    }
}

#[doc(inline)]
pub use crate::{errors::*, main::*};

*/
