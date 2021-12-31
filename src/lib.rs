#![no_implicit_prelude]
#![allow(
    unused_imports,
    non_camel_case_types,
    macro_expanded_macro_exports_accessed_by_absolute_paths
)]


#![allow(
    clippy::comparison_chain,
    clippy::cargo,
    clippy::pedantic,
    clippy::trivially_copy_pass_by_ref
)]
#![warn(
    clippy::self_named_module_files,
    clippy::unseparated_literal_suffix,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cloned_instead_of_copied,
    clippy::create_dir,
    clippy::wildcard_imports,
    clippy::default_trait_access
)]


mod error;
mod float;
mod int;
mod str;

pub(crate) use ::{
    ambassador::Delegate,
    educe,
    std::{
        borrow::ToOwned,
        clone::Clone,
        convert::{AsMut, AsRef, From, Into, TryFrom, TryInto},
        default::Default,
        fmt::{Debug, Display},
        format,
        string::String,
        sync::{Arc, Mutex, RwLock},
    },
};
pub use {
    self::{error::*, float::*, int::*, str::*},
    ::{
        core::{self, option::Option, todo},
        eyre::{Report as Error, Result},
        std::{self, format as f},
    },
};

pub struct string {
    inner: Arc<std::string::String>,
}

impl From<&str> for string {
    fn from(str: &str) -> Self {
        Self {
            inner: std::sync::Arc::new(str.to_owned()),
        }
    }
}

impl From<String> for string {
    fn from(str: String) -> Self {
        Self {
            inner: std::sync::Arc::new(str),
        }
    }
}

impl Display for string {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&*self.inner, f)
    }
}

impl Debug for string {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&*self.inner, f)
    }
}

pub fn print(s: impl Display) {
    std::println!("{}", s);
}

pub fn pprint(s: impl Debug) {
    std::println!("{:#?}", s);
}

pub fn str(x: impl Display) -> string {
    format!("{}", x).into()
}

pub fn repr(x: impl Debug) -> String {
    format!("{:?}", x).into()
}

// #[derive(clap::Parser, Debug)]
// #[clap(about, author)]
// pub struct Args {
//     /// Optionally, specify a single year to run (instead of all).
//     pub year: Option<u32>,

//     /// Optionally, specify a single day to run (instead of all).
//     pub day: Option<u32>,
// }

// #[macro_rules_attribute(pyelude::main!)]
// pub fn main(args: {

// }) -> Result<()> {

//     }
// }

// one thing at a time?
