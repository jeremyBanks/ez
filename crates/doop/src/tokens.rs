use crate::*;

/// Convenience wrapper for a list of one or more [`TokenTree`]s.
#[derive(Debug, Clone)]
pub struct Tokens {
    // At least one of `tree`, `stream`, or `vec` must always be non-empty.
    // Other fields will be lazily initialized the first time they're needed,
    // or cleared out if any changes are made.
    tree: OnceCell<Option<TokenTree>>,
    stream: OnceCell<TokenStream>,
    vec: OnceCell<Vec<TokenTree>>,
    string: OnceCell<String>,
}

impl Tokens /* for Empty */ {
    /// Creates a new, empty, list.
    pub fn new() -> Tokens {
        Tokens {
            tree: OnceCell::new(),
            stream: OnceCell::new(),
            vec: OnceCell::with_value(Vec::new()),
            string: OnceCell::new(),
        }
    }
}

impl Default for Tokens {
    fn default() -> Tokens {
        Tokens::new()
    }
}

impl Tokens /* for TokenStream */ {
    pub fn from_stream(stream: impl Into<TokenStream>) -> Tokens {
        Tokens {
            tree: OnceCell::new(),
            stream: OnceCell::with_value(stream.into()),
            vec: OnceCell::new(),
            string: OnceCell::new(),
        }
    }

    pub fn stream(&self) -> &TokenStream {
        self.stream.get_or_init(|| self.vec().iter().cloned().collect())
    }

    pub fn mut_stream(&mut self) -> &mut TokenStream {
        self.stream();
        self.tree = OnceCell::new();
        self.vec = OnceCell::new();
        self.string = OnceCell::new();
        self.stream.get_mut().unwrap()
    }

    pub fn into_stream(self) -> TokenStream {
        self.stream();
        self.stream.into_inner().unwrap()
    }
}

impl From<Tokens> for TokenStream {
    fn from(tokens: Tokens) -> TokenStream {
        tokens.into_stream()
    }
}

impl AsRef<TokenStream> for Tokens {
    fn as_ref(&self) -> &TokenStream {
        self.stream()
    }
}

impl AsMut<TokenStream> for Tokens {
    fn as_mut(&mut self) -> &mut TokenStream {
        self.mut_stream()
    }
}

impl From<TokenStream> for Tokens {
    fn from(stream: TokenStream) -> Tokens {
        Tokens::from_stream(stream)
    }
}

impl Tokens /* for Vec<TokenTree> */ {
    pub fn from_vec(vec: Vec<TokenTree>) -> Tokens {
        Tokens {
            tree: OnceCell::new(),
            vec: OnceCell::with_value(vec),
            stream: OnceCell::new(),
            string: OnceCell::new(),
        }
    }

    pub fn vec(&self) -> &Vec<TokenTree> {
        self.vec.get_or_init(|| self.stream().clone().into_iter().collect())
    }

    pub fn mut_vec(&mut self) -> &mut Vec<TokenTree> {
        self.vec();
        self.stream = OnceCell::new();
        self.string = OnceCell::new();
        self.vec.get_mut().unwrap()
    }

    pub fn into_vec(self) -> Vec<TokenTree> {
        self.vec();
        self.vec.into_inner().unwrap()
    }
}

impl From<Tokens> for Vec<TokenTree> {
    fn from(tokens: Tokens) -> Vec<TokenTree> {
        tokens.into_vec()
    }
}

impl AsRef<Vec<TokenTree>> for Tokens {
    fn as_ref(&self) -> &Vec<TokenTree> {
        self.vec()
    }
}

impl AsMut<Vec<TokenTree>> for Tokens {
    fn as_mut(&mut self) -> &mut Vec<TokenTree> {
        self.mut_vec()
    }
}

impl From<Vec<TokenTree>> for Tokens {
    fn from(vec: Vec<TokenTree>) -> Tokens {
        Tokens::from_vec(vec)
    }
}

impl Tokens /* for Option<TokenTree> */ {
    pub fn from_tree(tree: TokenTree) -> Tokens {
        Tokens {
            tree: OnceCell::with_value(Some(tree)),
            vec: OnceCell::new(),
            stream: OnceCell::new(),
            string: OnceCell::new(),
        }
    }

    pub fn tree(&self) -> Option<&TokenTree> {
        self.tree
            .get_or_init(
                || if self.len() == 1 { Some(self.first().unwrap().clone()) } else { None },
            )
            .as_ref()
    }

    pub fn mut_tree(&mut self) -> Option<&mut TokenTree> {
        self.tree();
        self.vec = OnceCell::new();
        self.stream = OnceCell::new();
        self.string = OnceCell::new();
        self.tree.get_mut().and_then(|tree| tree.as_mut())
    }

    pub fn into_tree(self) -> Option<TokenTree> {
        self.tree();
        self.tree.into_inner().unwrap()
    }
}

impl From<TokenTree> for Tokens {
    fn from(tree: TokenTree) -> Tokens {
        Self::from_tree(tree)
    }
}

impl AsRef<Option<TokenTree>> for Tokens {
    fn as_ref(&self) -> &Option<TokenTree> {
        self.tree()
    }
}

impl AsMut<Option<TokenTree>> for Tokens {
    fn as_mut(&mut self) -> &mut Option<TokenTree> {
        self.mut_tree()
    }
}

impl TryFrom<Tokens> for Option<TokenTree> {
    type Error = ();
    fn try_from(tokens: Tokens) -> Option<TokenTree> {
        tokens.into_tree()
    }
}

impl Tokens /* for String */ {
    pub fn from_string(string: &str) -> Tokens {
        Tokens::from_stream(string.parse::<TokenStream>().unwrap())
    }

    pub fn string(&self) -> &String {
        self.string.get_or_init(|| self.stream().to_string())
    }

    pub fn into_string(self) -> String {
        self.string();
        self.string.into_inner().unwrap()
    }
}

impl From<Tokens> for String {
    fn from(tokens: Tokens) -> String {
        tokens.into_string()
    }
}

impl AsRef<String> for Tokens {
    fn as_ref(&self) -> &String {
        self.string()
    }
}

impl From<&str> for Tokens {
    fn from(string: &str) -> Tokens {
        Tokens::from_string(string)
    }
}

impl Tokens {
    pub fn is_empty(&self) -> bool {
        if let Some(vec) = self.vec.get() {
            vec.is_empty()
        } else if let Some(stream) = self.stream.get() {
            stream.is_empty()
        } else if let Some(Some(_)) = self.tree.get() {
            true
        } else {
            unreachable!()
        }
    }

    pub fn replace(&self, replacements: &HashMap<String, TokenStream>) -> Tokens {
        self.stream().replace(replacements).into()
    }

    pub fn len(&self) -> usize {
        self.vec().len()
    }

    pub fn first(&self) -> Option<&TokenTree> {
        self.vec().first()
    }

    pub fn last(&self) -> Option<&TokenTree> {
        self.vec().last()
    }

    pub fn get(&self, index: usize) -> Option<&TokenTree> {
        self.vec().get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenTree> {
        self.vec().iter()
    }

    pub fn ident(&self) -> Option<&Ident> {
        self.tree().and_then(|tt| tt.ident())
    }

    pub fn literal(&self) -> Option<&Literal> {
        self.tree().and_then(|tt| tt.literal())
    }

    pub fn punct(&self) -> Option<&Punct> {
        self.tree().and_then(|tt| tt.punct())
    }

    pub fn bracketed(&self) -> Option<Tokens> {
        self.tree().and_then(|tt| tt.bracketed())
    }

    pub fn braced(&self) -> Option<Tokens> {
        self.tree().and_then(|tt| tt.braced())
    }

    pub fn parenthesized(&self) -> Option<Tokens> {
        self.tree().and_then(|tt| tt.parenthesized())
    }

    pub fn split_lines(&self) -> Vec<&[TokenTree]> {
        let mut slices = Vec::new();
        let mut next_line_start_index = 0;
        for (i, tt) in self.vec().iter().enumerate() {
            match tt {
                TokenTree::Punct(punct) if punct.as_char() == ';' => {
                    slices.push(&self.vec()[next_line_start_index..=i]);
                    next_line_start_index = i + 1;
                }
                TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                    slices.push(&self.vec()[next_line_start_index..=i]);
                    next_line_start_index = i + 1;
                }
                _ => {}
            }
        }
        slices.push(&self.vec()[next_line_start_index..]);
        slices
    }

    pub fn split_commas(&self) -> Vec<&[TokenTree]> {
        let mut slices = Vec::new();
        let mut next_comma_start_index = 0;
        for (i, tt) in self.vec().iter().enumerate() {
            match tt {
                TokenTree::Punct(punct) if punct.as_char() == ',' => {
                    slices.push(&self.vec()[next_comma_start_index..=i]);
                    next_comma_start_index = i + 1;
                }
                _ => {}
            }
        }
        slices.push(&self.vec()[next_comma_start_index..]);
        slices
    }
}

impl Index<usize> for Tokens {
    type Output = TokenTree;

    fn index(&self, index: usize) -> &TokenTree {
        &self.vec()[index]
    }
}

impl IntoIterator for Tokens {
    type Item = TokenTree;
    type IntoIter = Box<dyn Iterator<Item = TokenTree>>;

    fn into_iter(self) -> Self::IntoIter {
        match self.into_members() {
            (_, Some(vec), _) => Box::new(vec.into_iter()),
            (Some(stream), _, _) => Box::new(stream.into_iter()),
            _ => unreachable!(),
        }
    }
}

impl Borrow<String> for Tokens {
    fn borrow(&self) -> &String {
        self.string()
    }
}

impl Hash for Tokens {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string().hash(state);
    }
}

impl PartialEq for Tokens {
    fn eq(&self, other: &Tokens) -> bool {
        self.string() == other.string()
    }
}

impl Eq for Tokens {}

impl Ord for Tokens {
    fn cmp(&self, other: &Tokens) -> Ordering {
        self.string().cmp(&other.string())
    }
}

impl PartialOrd for Tokens {
    fn partial_cmp(&self, other: &Tokens) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}

impl FromIterator<TokenTree> for Tokens {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = TokenTree>,
    {
        Tokens::from_vec(iter.into_iter().collect())
    }
}

impl From<Group> for Tokens {
    fn from(group: Group) -> Self {
        Tokens::from_tree(TokenTree::Group(group))
    }
}

impl From<Punct> for Tokens {
    fn from(punct: Punct) -> Self {
        Tokens::from_tree(TokenTree::Punct(punct))
    }
}

impl From<Ident> for Tokens {
    fn from(ident: Ident) -> Self {
        Tokens::from_tree(TokenTree::Ident(ident))
    }
}

impl From<Literal> for Tokens {
    fn from(literal: Literal) -> Self {
        Tokens::from_tree(TokenTree::Literal(literal))
    }
}

impl<T: Into<Tokens>> AddAssign<T> for &mut Tokens {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();

        if !self.stream.get().is_some() && !self.vec.get().is_some() {
            self.vec();
        }

        self.string = OnceCell::new();
        self.tree = OnceCell::new();

        match (self.stream.get_mut(), self.vec.get_mut()) {
            (None, Some(vec)) => {
                vec.extend(rhs.into_iter());
            }
            (Some(stream), None) => {
                stream.extend(rhs.into_iter());
            }
            (Some(stream), Some(vec)) => {
                stream.extend(rhs.clone().into_iter());
                vec.extend(rhs.into_iter());
            }
            _ => unreachable!(),
        }
    }
}

impl Tokens {
    fn extend(&mut self, rhs: impl Into<Tokens>) {
        self += rhs;
    }
}
