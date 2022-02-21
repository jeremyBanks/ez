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
        ::{
            bytes::{Buf, BufMut, Bytes, BytesMut},
            crossbeam::{self, thread::scope as thread_scope},
            doop::doop,
            dpc_pariter::IteratorExt,
            euclid,
            indexmap::{IndexMap as OrderedMap, IndexSet as OrderedSet},
            itertools::Itertools,
            num_traits::*,
            parking_lot::{self, Mutex, RwLock},
            rand,
            rayon::{self, prelude::*},
            regex,
            serde_json::Value as Json,
            tokio,
            tracing::{self, debug, error, info, instrument, trace, warn},
        },
    };
}
