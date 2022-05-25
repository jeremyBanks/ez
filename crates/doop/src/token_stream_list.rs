//! For doop's macros, we need to allow various set-like operations on
//! lists of TokenStreams. (They're not sets: order and duplicates are
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
pub struct TokenStreamList {
    vec: Vec<TokenStream>,
}

impl TokenStreamList {
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn extend(&mut self, other: &TokenStreamList) {
        self.vec.extend(other.vec.iter().cloned())
    }

    pub fn union(&mut self, other: &TokenStreamList) {
        let set: HashSet<TokenStream> = self.vec.iter().cloned().collect();
        for item in other.vec.iter() {
            if !set.contains(item) {
                self.vec.push(item.clone());
            }
        }
    }

    pub fn intersect(&mut self, other: &TokenStreamList) {
        let set: HashSet<&TokenStream> = other.vec.iter().collect();
        self.vec.retain(|item| set.contains(item));
    }

    pub fn remove(&mut self, other: &TokenStreamList) {
        let set: HashSet<&TokenStream> = other.vec.iter().collect();
        self.vec.retain(|item| !set.contains(item));
    }
}

impl From<Vec<TokenStream>> for TokenStreamList {
    fn from(vec: Vec<TokenStream>) -> Self {
        TokenStreamList { vec }
    }
}

impl Into<Vec<TokenStream>> for TokenStreamList {
    fn into(self) -> Vec<TokenStream> {
        self.vec
    }
}

impl Deref for TokenStreamList {
    type Target = Vec<TokenStream>;

    fn deref(&self) -> &Vec<TokenStream> {
        &self.vec
    }
}

impl AsRef<[TokenStream]> for TokenStreamList {
    fn as_ref(&self) -> &[TokenStream] {
        &self.vec
    }
}

impl AddAssign<&TokenStreamList> for TokenStreamList {
    fn add_assign(&mut self, other: &TokenStreamList) {
        self.extend(other)
    }
}

impl SubAssign<&TokenStreamList> for TokenStreamList {
    fn sub_assign(&mut self, other: &TokenStreamList) {
        self.remove(other)
    }
}

impl BitAndAssign<&TokenStreamList> for TokenStreamList {
    fn bitand_assign(&mut self, other: &TokenStreamList) {
        self.intersect(other)
    }
}

impl BitOrAssign<&TokenStreamList> for TokenStreamList {
    fn bitor_assign(&mut self, other: &TokenStreamList) {
        self.union(other)
    }
}

impl Add for &TokenStreamList {
    type Output = TokenStreamList;
    fn add(self, other: &TokenStreamList) -> TokenStreamList {
        self.clone().tap_mut(|result| *result += other)
    }
}

impl Sub for &TokenStreamList {
    type Output = TokenStreamList;
    fn sub(self, other: &TokenStreamList) -> TokenStreamList {
        self.clone().tap_mut(|result| *result -= other)
    }
}

impl BitAnd for &TokenStreamList {
    type Output = TokenStreamList;
    fn bitand(self, other: &TokenStreamList) -> TokenStreamList {
        self.clone().tap_mut(|result| *result &= other)
    }
}

impl BitOr for &TokenStreamList {
    type Output = TokenStreamList;
    fn bitor(self, other: &TokenStreamList) -> TokenStreamList {
        self.clone().tap_mut(|result| *result |= other)
    }
}
