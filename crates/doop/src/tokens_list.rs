//! For doop's macros, we need to allow various set-like operations on
//! lists of Tokens. (They're not sets: order and duplicates are
//! generally preserved.) The main operations are:
//!
//! - `L + R`: extend, creates a list containing all elements of `L` followed by
//!   all elements of `R`.
//! - `L | R`: union, creates a list containing all elements of `L` followed by
//!   all elements of `R` which were not already in `L`.
//! - `L & R`: intersection, creates a list containing all elements of `L` which
//!   are also in `R`.
//! - `L - R`: removal, creates a list containing all elements of `L` which are
//!   not in `R`.

use crate::*;

#[derive(Debug, Clone, Default)]
pub struct TokensList {
    vec: Vec<LikeString<TokenStream>>,
}

impl TokensList {
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn extend(&mut self, other: &TokensList) {
        self.vec.extend(other.vec.iter().cloned())
    }

    pub fn union(&mut self, other: &TokensList) {
        let set: HashSet<LikeString<TokenStream>> = self.vec.iter().cloned().collect();
        for item in &other.vec {
            if !set.contains(item) {
                self.vec.push(item.clone());
            }
        }
    }

    pub fn intersect(&mut self, other: &TokensList) {
        let set: HashSet<&LikeString<TokenStream>> = other.vec.iter().collect();
        self.vec.retain(|item| set.contains(item));
    }

    pub fn remove(&mut self, other: &TokensList) {
        let set: HashSet<&LikeString<TokenStream>> = other.vec.iter().collect();
        self.vec.retain(|item| !set.contains(item));
    }
}
