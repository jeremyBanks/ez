//! For doop's macros, we need to support replacement of an identifier
//! with an arbitrary token stream, recursively, within another TokenStream.
//!
//! We also need frozen TokenStreams that are hashable and comparable, based
//! on their string representation (spans are ignored).

use crate::*;

#[derive(Debug, Clone, Default)]
pub struct TokenStream {
    stream: TokenStream2,
    vec: Vec<proc_macro2::TokenTree>,
    string: String,
}

impl TokenStream {
    pub fn replace(&self, replacements: &HashMap<Ident, TokenStream2>) -> TokenStream {
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

    pub fn bracketed(&self) -> Option<TokenStream> {
        self.only().and_then(|tt| tt.bracketed())
    }

    pub fn braced(&self) -> Option<TokenStream> {
        self.only().and_then(|tt| tt.braced())
    }

    pub fn parenthesized(&self) -> Option<TokenStream> {
        self.only().and_then(|tt| tt.parenthesized())
    }

    pub fn lines(&self) -> Vec<&[TokenTree]> {
        let mut result = Vec::new();
        let mut next_line_start_index = 0;
        for (i, tt) in self.vec.iter().enumerate() {
            match tt {
                TokenTree::Punct(punct) =>
                    if punct.as_char() == ';' {
                        result.push(&self.vec[next_line_start_index..=i]);
                        next_line_start_index = i + 1;
                    },
                TokenTree::Group(group) =>
                    if group.delimiter() == Delimiter::Brace {
                        result.push(&self.vec[next_line_start_index..=i]);
                        next_line_start_index = i + 1;
                    },
                _ => {}
            }
        }
        result.push(&self.vec[next_line_start_index..]);
        result
    }
}

impl TokenStream2Ext for TokenStream2 {}
pub trait TokenStream2Ext: Borrow<TokenStream2> + BorrowMut<TokenStream2> {
    fn replace(&self, replacements: &HashMap<Ident, TokenStream2>) -> TokenStream2 {
        let mut output = TokenStream2::new();
        for tree in self.borrow().clone() {
            match &tree {
                TokenTree::Ident(ident) =>
                    if let Some(replacement) = replacements.get(&ident) {
                        output.extend(replacement.clone());
                    } else {
                        output.extend(Some(tree));
                    },
                TokenTree::Group(group) => {
                    output.extend(Some(TokenTree::Group(Group::new(
                        group.delimiter(),
                        group.stream().replace(replacements),
                    ))));
                }
                _ => output.extend(Some(tree)),
            }
        }
        output
    }
}

impl Index<usize> for TokenStream {
    type Output = proc_macro2::TokenTree;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl IntoIterator for TokenStream {
    type Item = TokenTree;
    type IntoIter = std::vec::IntoIter<TokenTree>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl Hash for TokenStream {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string.hash(state);
    }
}

impl PartialEq for TokenStream {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl Eq for TokenStream {}

impl Ord for TokenStream {
    fn cmp(&self, other: &Self) -> Ordering {
        self.string.cmp(&other.string)
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl PartialOrd for TokenStream {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<TokenStream2> for TokenStream {
    fn from(stream: TokenStream2) -> Self {
        let vec = stream.clone().into_iter().collect();
        let string = stream.to_string();
        Self { stream, vec, string }
    }
}

impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = TokenTree>,
    {
        Self::from(Vec::from_iter(iter.into_iter()))
    }
}

impl From<Vec<TokenTree>> for TokenStream {
    fn from(vec: Vec<TokenTree>) -> Self {
        let stream = TokenStream2::from_iter(vec.iter().cloned());
        let string = stream.to_string();
        Self { stream, vec, string }
    }
}

impl Deref for TokenStream {
    type Target = TokenStream2;

    fn deref(&self) -> &TokenStream2 {
        &self.stream
    }
}

impl Into<TokenStream2> for TokenStream {
    fn into(self) -> TokenStream2 {
        self.stream
    }
}

impl Into<Vec<TokenTree>> for TokenStream {
    fn into(self) -> Vec<TokenTree> {
        self.vec
    }
}

impl AsRef<Vec<TokenTree>> for TokenStream {
    fn as_ref(&self) -> &Vec<TokenTree> {
        &self.vec
    }
}

impl AsRef<TokenStream2> for TokenStream {
    fn as_ref(&self) -> &TokenStream2 {
        &self.stream
    }
}

impl AsRef<str> for TokenStream {
    fn as_ref(&self) -> &str {
        &self.string
    }
}
