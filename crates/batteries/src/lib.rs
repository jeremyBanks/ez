#![warn(unused_crate_dependencies)]

pub mod prelude {
    pub use crate::batteries::*;
}

pub mod batteries {
    #[doc(no_inline)]
    pub use {
        std::{
            collections::{
                BTreeMap as SortedMap, BTreeSet as SortedSet, HashMap, HashSet, VecDeque as Deque,
            },
            fmt::{Debug, Display},
            process::Command,
            rc::Rc,
            sync::Arc,
            time::Duration,
        },
        {
            blake3,
            bytes::{Buf, BufMut, Bytes, BytesMut},
            clap,
            crossbeam::{self, thread::scope as thread_scope},
            crossterm::{self, tty::IsTty},
            csv, digest,
            doop::{doop, dooped},
            dpc_pariter::IteratorExt,
            either::Either,
            euclid,
            indexmap::{IndexMap as OrderedMap, IndexSet as OrderedSet},
            itertools::Itertools,
            match_cfg::match_cfg,
            num_traits::*,
            once_cell,
            parking_lot::{self, Mutex, RwLock},
            rand,
            rayon::{self, prelude::*},
            regex, reqwest, serde,
            serde_json::{self, Value as Json},
            sha2, sha3,
            tap::{self, Tap},
            tokio,
            toml::{self, Value as Toml},
            tracing::{self, debug, error, info, instrument, trace, warn},
            unicode_segmentation,
            url::{self, Url},
            walkdir::{self, WalkDir},
        },
    };
}
