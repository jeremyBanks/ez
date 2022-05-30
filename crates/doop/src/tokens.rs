//! The `Tokens` type provides a collection of convenience methods, wrapping a
//! [`TokenStream`] and/or equivalent .
use crate::*;

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
        self.tree.get_mut().and_then(Option::as_mut)
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

    pub fn replace_deep(&self, replacements: &HashMap<String, Tokens>) -> Tokens {
        let mut output = Tokens::new();
        for tree in self.iter() {
            match &tree {
                TokenTree::Ident(ident) =>
                    if let Some(replacement) = replacements.get(&ident.to_string()) {
                        output.extend(replacement.clone());
                    } else {
                        output.extend(tree.clone());
                    },
                TokenTree::Group(group) => {
                    output.extend(Group::new(
                        group.delimiter(),
                        Tokens::from(group.stream()).replace_deep(replacements).into(),
                    ));
                }
                _ => output.extend(tree.clone()),
            }
        }
        output
    }

    pub fn replace_shallow(&self, replacements: &HashMap<String, Tokens>) -> Tokens {
        let mut output = Tokens::new();
        for tree in self.iter() {
            match &tree {
                TokenTree::Ident(ident) =>
                    if let Some(replacement) = replacements.get(&ident.to_string()) {
                        output.extend(replacement.clone().into_tokens());
                    } else {
                        output.extend(tree.clone().into_tokens());
                    },
                _ => output.extend(tree.clone().into_tokens()),
            }
        }
        output
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
    pub fn group(&self) -> Option<&Group> {
        if let Some(TokenTree::Group(group)) = self.tree() {
            Some(group)
        } else {
            None
        }
    }

    pub fn punct(&self) -> Option<&Punct> {
        if let Some(TokenTree::Punct(punct)) = self.tree() {
            Some(punct)
        } else {
            None
        }
    }

    pub fn ident(&self) -> Option<&Ident> {
        if let Some(TokenTree::Ident(ident)) = self.tree() {
            Some(ident)
        } else {
            None
        }
    }

    pub fn literal(&self) -> Option<&Literal> {
        if let Some(TokenTree::Literal(literal)) = self.tree() {
            Some(literal)
        } else {
            None
        }
    }

    pub fn bracketed(&self) -> Option<Tokens> {
        let group = self.group()?;
        if group.delimiter() == Delimiter::Brace {
            Some(group.stream().into())
        } else {
            None
        }
    }

    pub fn braced(&self) -> Option<Tokens> {
        let group = self.group()?;
        if group.delimiter() == Delimiter::Brace {
            Some(group.stream().into())
        } else {
            None
        }
    }

    pub fn parenthesized(&self) -> Option<Tokens> {
        let group = self.group()?;
        if group.delimiter() == Delimiter::Parenthesis {
            Some(group.stream().into())
        } else {
            None
        }
    }

    pub fn error<T: From<Tokens>, M: AsRef<str>>(&self, message: M) -> T {
        let span = self.first().map_or(Span::call_site(), TokenTree::span);

        let ident = Ident::new("compile_error", span);

        let mut punct = Punct::new('!', Spacing::Alone);
        punct.set_span(span);

        let mut group = Group::new(
            Delimiter::Parenthesis,
            TokenStream::from(TokenTree::Literal(Literal::string(message.as_ref()))),
        );
        group.set_span(span);

        Tokens::from(vec![
            TokenTree::Ident(ident),
            TokenTree::Punct(punct),
            TokenTree::Group(group),
        ])
        .into()
    }

    pub fn split_lines(&self) -> impl IntoIterator<Item = Tokens> {
        let vec = self.vec();
        let mut lines = Vec::new();
        let mut next_line_start_index = 0;
        for (i, tt) in vec.iter().enumerate() {
            match tt {
                TokenTree::Punct(punct) if punct.as_char() == ';' => {
                    lines.push(Tokens::from_iter(vec[next_line_start_index..=i].iter().cloned()));
                    next_line_start_index = i + 1;
                }
                TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                    lines.push(Tokens::from_iter(vec[next_line_start_index..=i].iter().cloned()));
                    next_line_start_index = i + 1;
                }
                _ => {}
            }
        }
        lines.push(Tokens::from_iter(vec[next_line_start_index..].iter().cloned()));

        lines.into_iter()
    }

    pub fn split_commas(&self) -> impl IntoIterator<Item = Tokens> {
        let vec = self.vec();
        let mut slices = Vec::new();
        let mut next_comma_start_index = 0;
        for (i, tt) in vec.iter().enumerate() {
            match tt {
                TokenTree::Punct(punct) if punct.as_char() == ',' => {
                    slices.push(Tokens::from_iter(vec[next_comma_start_index..=i].iter().cloned()));
                    next_comma_start_index = i + 1;
                }
                _ => {}
            }
        }

        slices.push(Tokens::from_iter(vec[next_comma_start_index..].iter().cloned()));

        slices.into_iter()
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
        if let Some(Some(_)) = self.tree.get() {
            Box::new(self.into_tree().into_iter())
        } else if self.vec.get().is_some() {
            Box::new(self.into_vec().into_iter())
        } else if self.stream.get().is_some() {
            Box::new(self.into_stream().into_iter())
        } else {
            unreachable!()
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
        self.string().cmp(other.string())
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

impl Tokens {
    pub fn extend(&mut self, rhs: impl Into<Tokens>) {
        let rhs = rhs.into_tokens();

        if self.stream.get().is_none() && self.vec.get().is_none() {
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

pub trait IntoTokens: Sized {
    fn into_tokens(self) -> Tokens;

    fn error<M: AsRef<str>, R>(self, message: M) -> Result<R, Tokens> {
        Err(Tokens::error(&self.into_tokens(), message))
    }
}

impl<T: Into<Tokens>> IntoTokens for T {
    fn into_tokens(self) -> Tokens {
        self.into()
    }
}
