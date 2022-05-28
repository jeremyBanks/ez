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
    vec: Vec<Tokens>,
}

impl TokensList {
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn extend(&mut self, other: &TokensList) {
        self.vec.extend(other.vec.iter().cloned())
    }

    pub fn union(&mut self, other: &TokensList) {
        let set: HashSet<Tokens> = self.vec.iter().cloned().collect();
        for item in other.vec.iter() {
            if !set.contains(item) {
                self.vec.push(item.clone());
            }
        }
    }

    pub fn intersect(&mut self, other: &TokensList) {
        let set: HashSet<&Tokens> = other.vec.iter().collect();
        self.vec.retain(|item| set.contains(item));
    }

    pub fn remove(&mut self, other: &TokensList) {
        let set: HashSet<&Tokens> = other.vec.iter().collect();
        self.vec.retain(|item| !set.contains(item));
    }
}

impl From<Vec<Tokens>> for TokensList {
    fn from(vec: Vec<Tokens>) -> Self {
        TokensList { vec }
    }
}

impl Into<Vec<Tokens>> for TokensList {
    fn into(self) -> Vec<Tokens> {
        self.vec
    }
}

impl Deref for TokensList {
    type Target = Vec<Tokens>;

    fn deref(&self) -> &Vec<Tokens> {
        &self.vec
    }
}

impl AsRef<[Tokens]> for TokensList {
    fn as_ref(&self) -> &[Tokens] {
        &self.vec
    }
}

impl AddAssign<&TokensList> for TokensList {
    fn add_assign(&mut self, other: &TokensList) {
        self.extend(other)
    }
}

impl SubAssign<&TokensList> for TokensList {
    fn sub_assign(&mut self, other: &TokensList) {
        self.remove(other)
    }
}

impl BitAndAssign<&TokensList> for TokensList {
    fn bitand_assign(&mut self, other: &TokensList) {
        self.intersect(other)
    }
}

impl BitOrAssign<&TokensList> for TokensList {
    fn bitor_assign(&mut self, other: &TokensList) {
        self.union(other)
    }
}

impl Add for &TokensList {
    type Output = TokensList;
    fn add(self, other: &TokensList) -> TokensList {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl Sub for &TokensList {
    type Output = TokensList;
    fn sub(self, other: &TokensList) -> TokensList {
        let mut result = self.clone();
        result -= other;
        result
    }
}

impl BitAnd for &TokensList {
    type Output = TokensList;
    fn bitand(self, other: &TokensList) -> TokensList {
        let mut result = self.clone();
        result &= other;
        result
    }
}

impl BitOr for &TokensList {
    type Output = TokensList;
    fn bitor(self, other: &TokensList) -> TokensList {
        let mut result = self.clone();
        result |= other;
        result
    }
}
