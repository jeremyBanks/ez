use crate::*;

/// An immutable wrapper around a TokenStream, adding string-based
/// equality and hashing, and some other useful methods.
#[derive(Debug, Clone, Default)]
pub struct TokenStream {
    stream: TokenStream2,
    vec: Vec<proc_macro2::TokenTree>,
    string: String,
}

impl TokenStream {
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn first(&self) -> Option<TokenTree> {
        self.vec.first().cloned()
    }

    pub fn last(&self) -> Option<TokenTree> {
        self.vec.last().cloned()
    }

    pub fn get(&self, index: usize) -> Option<TokenTree> {
        self.vec.get(index).cloned()
    }

    pub fn replace<Replacements>(&self, replacements: &Replacements) -> TokenStream
    where
        Replacements: HashMap<Ident, Replacement>,
        Replacement: TokenStream2,
    {
        fn replace2<'a>(input: &TokenStream2, replacements: &Replacements) -> TokenStream2 {
            for mut tree in input {
                match tree {
                    TokenTree::Ident(ident) =>
                        if let Some(replacement) = replacements.get(&ident) {
                            output.extend(replacement);
                        } else {
                            output.push(tree);
                        },
                    TokenTree::Group(group) => {
                        output.push(TokenTree::Group(Group::new(
                            group.delimiter(),
                            replace2(&group.stream(), replacements),
                        )));
                    }
                    _ => output.push(tree),
                }
            }
            output
        }

        replace2(self.as_ref(), replacements).into()
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

impl PartialOrd for TokenStream {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Deref for TokenStream {
    type Target = TokenStream2;

    fn deref(&self) -> &TokenStream2 {
        &self.stream
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
    fn from_iter(iter: impl IntoIterator<Item = TokenTree>) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl From<Vec<proc_macro2::TokenTree>> for TokenStream {
    fn from(vec: Vec<proc_macro2::TokenTree>) -> Self {
        let stream = vec.iter().cloned().collect();
        let string = stream.to_string();
        Self { stream, vec, string }
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

impl AsRef<[proc_macro2::TokenTree]> for TokenStream {
    fn as_ref(&self) -> &[proc_macro2::TokenTree] {
        &self.vec
    }
}

impl AsRef<TokenStream2> for TokenStream {
    fn as_ref(&self) -> &TokenStream2 {
        &self.stream
    }
}
