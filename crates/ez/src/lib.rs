#![doc = include_str!("../README.md")]

#[doc(inline)]
pub use ::ez_core::{throw, throws, try_throws, Error};
#[doc(hidden)]
pub use ::ez_main::ly;
#[doc(inline)]
pub use ::ez_main::main;
pub use {
    ::ezio::{file, random, stdio, string},
    ::implicint::{int, Int},
};

pub mod prelude {
    #[doc(inline)]
    pub use ::batteries::prelude::*;
    pub use ::ez_main::main;
    #[doc(inline)]
    pub use ::ezio::prelude::*;
}

pub use ::batteries::batteries;

#[doc(hidden)]
pub mod __ {
    //! **⚠️ INTERNAL! DO NOT USE!**
    //!
    //! This should not be considered part of this crate's public API.
    //!
    //! The contents are public only due to internal implementation
    //! requirements.
    #[doc(inline)]
    pub use ::ez_core::__::*;
    #[doc(inline)]
    pub use ::ez_main::__::*;
}

#[cfg(doc)]
pub mod r#pub {
    //! Documents and posts, for reading by humans.
    //!
    //! This module only contains documentation and can not be imported.

    macro_rules! docs {
        ($( $ident:ident ),* $(,)?) => {
            $(
                pub mod $ident {
                    //! [🔗][self#!]
                    //!
                    //! <details>
                    //! <summary>&nbsp;</summary>
                    //! <br />
                    //! <div id="!"></div>
                    //!
                    #![doc = include_str!(concat!("../docs/", stringify!($ident), ".md"))]
                    //!
                    //! <br /><br /><br /><br /><br /><br /><br /><br />
                    //! <br /><br /><br /><br /><br /><br /><br /><br />
                    //! <br /><br /><br /><br /><br /><br /><br /><br />
                    //! <br /><br /><br /><br /><br /><br /><br /><br />
                    //! </details>
                    use super::*;
                    use super::super::*;
                }
            )*
        }
    }

    docs! {
        introducing_doop,
    }
}
