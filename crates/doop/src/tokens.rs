use crate::*;

/// A frozen list of tokens.
#[derive(Debug, Clone)]
pub struct Tokens {
    stream: OnceCell<TokenStream2>,
    vec: OnceCell<Vec<TokenTree>>,
    string: OnceCell<String>,
}

impl Tokens {
    pub fn new() -> Self {
        Tokens::from_members(None, Some(Vec::new()), Some(String::new()))
    }

    fn from_members(
        stream: Option<TokenStream2>,
        vec: Option<Vec<TokenTree>>,
        string: Option<String>,
    ) -> Self {
        Tokens {
            stream: stream.map(OnceCell::with_value).unwrap_or_else(OnceCell::new),
            vec: vec.map(OnceCell::with_value).unwrap_or_else(OnceCell::new),
            string: string.map(OnceCell::with_value).unwrap_or_else(OnceCell::new),
        }
    }

    fn members(&self) -> (Option<&TokenStream2>, Option<&Vec<TokenTree>>, Option<&String>) {
        (self.stream.get(), self.vec.get(), self.string.get())
    }

    fn mut_members(
        &mut self,
    ) -> (Option<&mut TokenStream2>, Option<&mut Vec<TokenTree>>, Option<&mut String>) {
        (self.stream.get_mut(), self.vec.get_mut(), self.string.get_mut())
    }

    fn into_members(self) -> (Option<TokenStream2>, Option<Vec<TokenTree>>, Option<String>) {
        (self.stream.into_inner(), self.vec.into_inner(), self.string.into_inner())
    }
}

// TODO: append methods

impl Default for Tokens {
    fn default() -> Tokens {
        Tokens::new()
    }
}

impl Tokens {
    pub fn from_stream(stream: impl Into<TokenStream2>) -> Tokens {
        Tokens {
            stream: OnceCell::with_value(stream.into()),
            vec: OnceCell::new(),
            string: OnceCell::new(),
        }
    }

    pub fn stream(&self) -> &TokenStream2 {
        self.stream.get_or_init(|| self.vec().iter().cloned().collect())
    }

    pub fn mut_stream(&mut self) -> &mut TokenStream2 {
        self.stream();
        self.vec = OnceCell::new();
        self.string = OnceCell::new();
        self.stream.get_mut().unwrap()
    }

    pub fn into_stream(self) -> TokenStream2 {
        self.stream();
        self.stream.into_inner().unwrap()
    }
}

impl From<Tokens> for TokenStream2 {
    fn from(tokens: Tokens) -> TokenStream2 {
        tokens.into_stream()
    }
}

impl AsRef<TokenStream2> for Tokens {
    fn as_ref(&self) -> &TokenStream2 {
        self.stream()
    }
}

impl AsMut<TokenStream2> for Tokens {
    fn as_mut(&mut self) -> &mut TokenStream2 {
        self.mut_stream()
    }
}

impl From<TokenStream2> for Tokens {
    fn from(stream: TokenStream2) -> Self {
        Tokens::from_stream(stream)
    }
}

impl Tokens {
    pub fn from_vec(vec: Vec<TokenTree>) -> Tokens {
        Tokens { vec: OnceCell::with_value(vec), stream: OnceCell::new(), string: OnceCell::new() }
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
    fn from(vec: Vec<TokenTree>) -> Self {
        Tokens::from_vec(vec)
    }
}

impl Tokens {
    pub fn from_string(string: String) -> Tokens {
        Tokens::from_stream(string.parse::<TokenStream2>().unwrap())
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

impl From<String> for Tokens {
    fn from(string: String) -> Self {
        Tokens::from_string(string)
    }
}

impl Tokens {
    pub fn is_empty(&self) -> bool {
        match self.members() {
            (_, Some(vec), _) => vec.is_empty(),
            (Some(stream), _, _) => stream.is_empty(),
            _ => unreachable!(),
        }
    }

    pub fn replace(&self, replacements: &HashMap<Ident, TokenStream2>) -> Tokens {
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

    pub fn only(&self) -> Option<&TokenTree> {
        if self.len() == 1 {
            self.first()
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&TokenTree> {
        self.vec().get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenTree> {
        self.vec().iter()
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
