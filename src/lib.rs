#![warn(unused_crate_dependencies)]
#![doc = include_str!("../README.md")]

pub mod main;

#[cfg(feature = "ze")]
/// Too much sugar: [`#[main]`][macro@main] + [`use ez::ze::*`][module@ze]
pub use ez_internal::ze;

#[doc(inline)]
pub use crate::main::*;

#[cfg(feature = "ze")]
/// This module re-exports a large collection of popular crates in the Rust
/// ecosystem. Enabling this feature (`ze`) adds a ton of dependencies and
/// will probably slow down your first build considerably.
pub mod ze {
    #[cfg(feature = "alloc")]
    pub extern crate alloc;
    #[cfg(feature = "core")]
    pub extern crate core;
    #[cfg(feature = "std")]
    pub extern crate std;

    pub use {
        crate::{self as ez},
        alloc::{rc::Rc, sync::Arc},
        core::fmt::{Debug, Display},
        std::{
            collections::{
                BTreeMap as SortedMap, BTreeSet as SortedSet, HashMap, HashSet, VecDeque as Deque,
            },
            path::{Path, PathBuf},
            process::Command,
            time::{Duration, Instant},
        },
        ::{
            atty::{self},
            boolinator::{self, Boolinator},
            bytes::{self, Bytes, BytesMut},
            crossbeam::{self, thread::scope as thread_scope},
            derive_more::{self, *},
            dpc_pariter::{self, IteratorExt},
            eyre::{self, eyre},
            indexmap::{self, IndexMap as OrderedMap, IndexSet as OrderedSet},
            itertools::{self, Itertools},
            lazy_static::{self, lazy_static},
            num_traits::{self, *},
            parking_lot::{self, Mutex, Once, RwLock},
            paste::{self},
            rand::{self},
            rayon::{self, prelude::*},
            regex::{self},
            reqwest::{self, blocking::get as fetch},
            serde::{self},
            serde_json::{self as json, json, Value as Json},
            socket2::{self},
            tokio::{self},
            tracing::{self, debug, error, info, instrument, trace, warn},
        },
    };
}
