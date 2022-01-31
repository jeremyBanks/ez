use {indexmap::IndexMap, std::borrow::Cow};

trait Main {
    type Args: MainArgs;
    type Env: MainEnv;
    type ExitStatus: MainExitStatus;

    fn call(&self, args: Self::Args, env: Self::Env) -> Self::ExitStatus;
}

impl<Args, Env, ExitStatus> Main for fn(Args, Env) -> ExitStatus
where
    Args: MainArgs,
    Env: MainEnv,
    ExitStatus: MainExitStatus,
{
    type Args = Args;
    type Env = Env;
    type ExitStatus = ExitStatus;

    fn call(&self, args: Self::Args, env: Self::Env) -> Self::ExitStatus {
        self(args, env)
    }
}

impl<Args, ExitStatus> Main for fn(Args) -> ExitStatus
where
    Args: MainArgs,
    ExitStatus: MainExitStatus,
{
    type Args = Args;
    type Env = CollectNothing;
    type ExitStatus = ExitStatus;

    fn call(&self, args: Self::Args, _: Self::Env) -> Self::ExitStatus {
        self(args)
    }
}

impl<ExitStatus> Main for fn() -> ExitStatus
where
    ExitStatus: MainExitStatus,
{
    type Args = CollectNothing;
    type Env = CollectNothing;
    type ExitStatus = ExitStatus;

    fn call(&self, _: Self::Args, _: Self::Env) -> Self::ExitStatus {
        self()
    }
}

struct CollectNothing;
impl<T> FromIterator<T> for CollectNothing {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        drop(iter);
        CollectNothing
    }
}

trait MainArgs: FromIterator<String> {}
impl<T> MainArgs for T where T: FromIterator<String> {}

trait MainEnv: FromIterator<(String, String)> {}
impl<T> MainEnv for T where T: FromIterator<(String, String)> {}

trait MainExitStatus {
    fn into_i32(self) -> i32;
}

impl<T, E> MainExitStatus for Result<T, E>
where
    T: MainExitStatus,
{
    fn into_i32(self) -> i32 {
        match self {
            Ok(status) => status.into_i32(),
            Err(_) => 101,
        }
    }
}

impl<T> MainExitStatus for Option<T>
where
    T: MainExitStatus,
{
    fn into_i32(self) -> i32 {
        match self {
            Some(status) => status.into_i32(),
            None => 0,
        }
    }
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

pub fn main() {
    fn run_main<F: Main>(f: F) -> ! {
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

        let status = f.call(args, env).into_i32();

        std::process::exit(status)
    }

    fn run_main_0<ExitStatus: MainExitStatus>(inner_main: fn() -> ExitStatus) {
        run_main(inner_main as fn() -> _);
    }

    fn run_main_1<Args: MainArgs, ExitStatus: MainExitStatus>(inner_main: fn(Args) -> ExitStatus) {
        run_main(inner_main as fn(_) -> _);
    }

    fn run_main_2<Args: MainArgs, Env: MainEnv, ExitStatus: MainExitStatus>(
        inner_main: fn(Args, Env) -> ExitStatus,
    ) {
        run_main(inner_main as fn(_, _) -> _);
    }

    run_main_0(|| {});
    run_main_1(|_args: Vec<_>| {});
    run_main_2(|_args: Vec<_>, _env: IndexMap<_, _>| {});
}
