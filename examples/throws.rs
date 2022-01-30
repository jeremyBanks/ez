use {
    eyre::{eyre, WrapErr},
    ez::throws,
    lazy_static::lazy_static,
    std::{cell::Cell, panic::PanicInfo},
};

macro_rules! catch_unhandled {
    { $($tt:tt)* } => {
        PANIC_HOOK_SUPPRESSIONS.with(|suppressions_cell| {
            suppressions_cell.set(suppressions_cell.get() + 1);
            let result = std::panic::catch_unwind(|| {
                    $($tt)*
            }).map_err(|err| {
                // if we can downcast to eyre, we can wrap it with
                // more context
                eprintln!("ERRRRR = {:#?}", err);
                if let Ok(err) = Box::new(err).downcast::<eyre::Report>() {
                    err.wrap_err("unhandled error panicked")
                } else {
                    eyre!("unhandled non-dynamic error panicked")
                }
            });
            suppressions_cell.set(suppressions_cell.get() - 1);
            result
        })
    };
}

thread_local! {
    static PANIC_HOOK_SUPPRESSIONS: Cell<i64> = Cell::new(0);
}

fn install_hook_suppressor() {
    static INSTALLED: std::sync::Once = std::sync::Once::new();
    INSTALLED.call_once(|| {
        let wrapped_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            PANIC_HOOK_SUPPRESSIONS.with(|suppressions_cell| {
                let suppressions = suppressions_cell.get();
                if suppressions == 0 {
                    wrapped_hook(info);
                }
            });
        }));
    });
}

#[throws]
fn main() {
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

    tracing_subscriber::util::SubscriberInitExt::init(tracing_subscriber::Layer::with_subscriber(
        tracing_error::ErrorLayer::default(),
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_target(false)
            .with_span_events(
                tracing_subscriber::fmt::format::FmtSpan::NEW
                    | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            )
            .finish(),
    ));

    install_hook_suppressor();

    // let _a = alice();
    // let _a = try_alice()?;
    // let _f = try_alice();

    // let _b = bob("1");
    // let _b = try_bob("threeve")?;
    // let _b = try_bob("3");

    let result = catch_unhandled!(Bar.foo());

    println!("I'm still standing! Despite: {:?}\n\n\n", result);

    result.unwrap();
}

/// This is Alice.
#[throws]
pub fn alice() -> i64 {
    try_bob("s")?
}

#[throws]
/// This is Bob.
pub fn bob(n: &str) -> i64 {
    n.parse()?
}

trait Foo {
    #[throws]
    fn foo(&self) -> i64;
}

struct Bar;

impl Foo for Bar {
    fn try_foo(&self) -> Result<i64, eyre::Report> {
        try_alice()
    }
}
