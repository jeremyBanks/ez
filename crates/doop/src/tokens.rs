//! For doop's macros, we need to support replacement of an identifier
//! with an arbitrary token stream, recursively, within another TokenStream.
//!
//! We also need frozen TokenStreams that are hashable and comparable, based
//! on their string representation (spans are ignored).

use crate::*;

#[derive(Debug, Clone, Default)]
pub struct Tokens {
    stream: TokenStream2,
    vec: Vec<proc_macro2::TokenTree>,
    string: String,
}

impl Tokens {
    pub fn replace(&self, replacements: &HashMap<Ident, TokenStream2>) -> Tokens {
        self.stream().replace(replacements).into()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn first(&self) -> Option<&TokenTree> {
        self.vec.first()
    }

    pub fn last(&self) -> Option<&TokenTree> {
        self.vec.last()
    }

    pub fn only(&self) -> Option<&TokenTree> {
        if self.vec.len() == 1 {
            self.first()
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&TokenTree> {
        self.vec.get(index)
    }

    pub fn iter(&self) -> std::slice::Iter<TokenTree> {
        self.vec.iter()
    }

    pub fn vec(&self) -> &Vec<TokenTree> {
        &self.vec
    }

    pub fn stream(&self) -> &TokenStream2 {
        &self.stream
    }

    pub fn ident(&self) -> Option<&Ident> {
        self.only().and_then(|tt| tt.ident())
    }

    pub fn literal(&self) -> Option<&Literal> {
        self.only().and_then(|tt| tt.literal())
    }

    pub fn punct(&self) -> Option<&Punct> {
        self.only().and_then(|tt| tt.punct())
    }

    pub fn bracketed(&self) -> Option<Tokens> {
        self.only().and_then(|tt| tt.bracketed())
    }

    pub fn braced(&self) -> Option<Tokens> {
        self.only().and_then(|tt| tt.braced())
    }

    pub fn parenthesized(&self) -> Option<Tokens> {
        self.only().and_then(|tt| tt.parenthesized())
    }

    pub fn split_lines(&self) -> Vec<&[TokenTree]> {
        let mut slices = Vec::new();
        let mut next_line_start_index = 0;
        for (i, tt) in self.vec.iter().enumerate() {
            match tt {
                TokenTree::Punct(punct) if punct.as_char() == ';' => {
                    slices.push(&self.vec[next_line_start_index..=i]);
                    next_line_start_index = i + 1;
                }
                TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                    slices.push(&self.vec[next_line_start_index..=i]);
                    next_line_start_index = i + 1;
                }
                _ => {}
            }
        }
        slices.push(&self.vec[next_line_start_index..]);
        slices
    }

    pub fn split_commas(&self) -> Vec<&[TokenTree]> {
        let mut slices = Vec::new();
        let mut next_comma_start_index = 0;
        for (i, tt) in self.vec.iter().enumerate() {
            match tt {
                TokenTree::Punct(punct) if punct.as_char() == ',' => {
                    slices.push(&self.vec[next_comma_start_index..=i]);
                    next_comma_start_index = i + 1;
                }
                _ => {}
            }
        }
        slices.push(&self.vec[next_comma_start_index..]);
        slices
    }
}

impl Index<usize> for Tokens {
    type Output = proc_macro2::TokenTree;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl IntoIterator for Tokens {
    type Item = TokenTree;
    type IntoIter = std::vec::IntoIter<TokenTree>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl Hash for Tokens {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string.hash(state);
    }
}

impl PartialEq for Tokens {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl Eq for Tokens {}

impl Ord for Tokens {
    fn cmp(&self, other: &Self) -> Ordering {
        self.string.cmp(&other.string)
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl PartialOrd for Tokens {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<TokenStream2> for Tokens {
    fn from(stream: TokenStream2) -> Self {
        let vec = stream.clone().into_iter().collect();
        let string = stream.to_string();
        Self { stream, vec, string }
    }
}

impl FromIterator<TokenTree> for Tokens {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = TokenTree>,
    {
        Self::from(Vec::from_iter(iter.into_iter()))
    }
}

impl From<Vec<TokenTree>> for Tokens {
    fn from(vec: Vec<TokenTree>) -> Self {
        let stream = TokenStream2::from_iter(vec.iter().cloned());
        let string = stream.to_string();
        Self { stream, vec, string }
    }
}

impl Deref for Tokens {
    type Target = TokenStream2;

    fn deref(&self) -> &TokenStream2 {
        &self.stream
    }
}

impl From<Tokens> for TokenStream2 {
    fn from(val: Tokens) -> Self {
        val.stream
    }
}

impl From<Tokens> for Vec<TokenTree> {
    fn from(val: Tokens) -> Self {
        val.vec
    }
}

impl AsRef<Vec<TokenTree>> for Tokens {
    fn as_ref(&self) -> &Vec<TokenTree> {
        &self.vec
    }
}

impl AsRef<TokenStream2> for Tokens {
    fn as_ref(&self) -> &TokenStream2 {
        &self.stream
    }
}

impl AsRef<str> for Tokens {
    fn as_ref(&self) -> &str {
        &self.string
    }
}
