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
            anymap, bitflags, blake3, bumpalo,
            bytes::{Buf, BufMut, Bytes, BytesMut},
            cfg_if, clap,
            crossbeam::{self, thread::scope as thread_scope},
            crossterm::{self, tty::IsTty},
            csv, digest,
            doop::{doop, dooped},
            dpc_pariter,
            dpc_pariter::IteratorExt,
            elor::Either,
            euclid, git2,
            hex::{decode as hex_decode, encode as hex_encode},
            hex_literal::hex,
            home::{cargo_home, home_dir, rustup_home},
            hyper, indexmap,
            indexmap::{IndexMap as OrderedMap, IndexSet as OrderedSet},
            itertools,
            itertools::Itertools,
            match_cfg::match_cfg,
            mio,
            munge::munge,
            nix, num_traits,
            num_traits::*,
            once_cell,
            parking_lot::{self, Mutex, RwLock},
            petgraph, proc_macro2, rand,
            rayon::{self, prelude::*},
            rdbc, regex, reqwest, scopeguard, serde,
            serde_json::{self, Value as Json},
            sha1, sha2, sha3, socket2, strum,
            tap::{self, Tap},
            tokio,
            toml::{self, Value as Toml},
            tracing::{self, debug, error, info, instrument, trace, warn},
            typed_arena, unicode_segmentation,
            url::{self, Url},
            walkdir::{self, WalkDir},
        },
    };
}
