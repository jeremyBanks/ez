#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use ez_internal::internal as __;
use ez_internal::publish;

publish! {
    use ez_internal::throw;
    docs from throw;
    failing example throw;
}

publish! {
    use ez_proc_macro::main;
    docs from main;
    include example main_noop;
    include example home;
    include example tokio;
}

publish! {
    use ez_proc_macro::try_throws;
    docs from try_throws;
    failing example try_throws;
}

publish! {
    use ez_proc_macro::throws;
    docs from throws;
}

/// A dynamic error reporting type. This is a re-export of
/// [`eyre::Report`](https://docs.rs/eyre/latest/eyre/struct.Report.html) from
/// [the `eyre` package](https://docs.rs/eyre).
///
/// ---
#[doc(inline)]
pub use __::eyre::Report as Error;

/// You may `use ez::prelude::*;`
pub mod prelude {
    #[doc(no_inline)]
    pub use ezio::prelude::*;

    #[doc(no_inline)]
    pub use crate::{io as ezio, main, throw, throws, try_throws};
}

/// This is a re-export of [the `ezio` package](https://docs.rs/ezio).
///
/// ---
#[doc(inline)]
pub use __::ezio as io;
