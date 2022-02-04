#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
pub extern crate alloc;
#[cfg(feature = "core")]
pub extern crate core;
#[cfg(feature = "std")]
pub extern crate std;

pub mod main;

#[doc(inline)]
pub use crate::main::*;

pub mod prelude {
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
            boolinator::{self, Boolinator},
            bytes::{self, Bytes, BytesMut},
            clap::{self},
            crossbeam::{self},
            derive_more::{self, *},
            dpc_pariter::{self, IteratorExt},
            eyre::{self, eyre},
            indexmap::{self, IndexMap as OrderedMap, IndexSet as OrderedSet},
            itertools::{self, Itertools},
            num_traits::{self, *},
            parking_lot::{self, Mutex, Once, RwLock},
            rayon::{self, prelude::*},
            serde::{self, Deserialize, Serialize},
            socket2::{self},
            thiserror::{self, Error},
            tokio::{self},
            tracing::{self, debug, error, info, trace, warn, instrument},
        },
    };
}
