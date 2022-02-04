#![warn(unused_crate_dependencies)]
#![doc = include_str!("../README.md")]

pub extern crate alloc;
pub extern crate core;
pub extern crate std;

pub mod main;

#[doc(hidden)]
#[cfg(feature = "pz")]
pub use ez_internal::pz;

#[doc(inline)]
pub use crate::main::*;

#[doc(hidden)]
#[cfg(feature = "pz")]
pub mod pz {
    pub use {
        crate::{self as ez},
        alloc::{self},
        core::{self},
        std::{
            self,
            collections::{
                BTreeMap as SortedMap, BTreeSet as SortedSet, HashMap, HashSet, VecDeque as Deque,
            },
            fmt::{Debug, Display},
            path::{Path, PathBuf},
            rc::Rc,
            sync::Arc,
            time::{Duration, Instant},
        },
        ::{
            atty::{self},
            boolinator::{self, Boolinator},
            bytes::{self, Bytes, BytesMut},
            clap::{self},
            crossbeam::{self},
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
            serde::{self, Deserialize, Serialize},
            serde_json::{self as json, json, Value as Json},
            socket2::{self},
            thiserror::{self, Error},
            tokio::{self},
            tracing::{self, debug, error, info, instrument, trace, warn},
        },
    };
}
