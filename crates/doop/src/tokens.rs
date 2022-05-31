use crate::*;

#[derive(Debug, Clone)]
pub struct Tokens {
    stream: TokenStream,
    string: OnceCell<String>,
}

struct AsToString<'inner, Inner: Display>(&'inner Inner);

impl Display for AsToString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Ord for AsToString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.to_string().cmp(&other.0.to_string())
    }
}

impl Tokens {
    pub const fn new() -> Tokens {
        Tokens { stream: TokenStream::new(), string: OnceCell::new() }
    }

    pub fn new_ident(ident: impl AsRef<str>, span: impl Into<Option<Span>>) -> Tokens {
        TokenStream::from_iter([
            TokenTree::Ident(
                Ident::new(ident.as_ref(), span.into().unwrap_or_else(Span::call_site))
            )
        ]).into_tokens()
    }

    pub fn new_group(
        delimiter: char,
        body: impl ToTokens,
        span: impl Into<Option<Span>>,
    ) -> Tokens {
        let delimiter = match delimiter {
            '(' => Delimiter::Parenthesis,
            '[' => Delimiter::Bracket,
            '{' => Delimiter::Brace,
            ' ' => Delimiter::None,
            _ => panic!("invalid group delimiter"),
        };

        let mut group = Group::new(delimiter, body.to_tokens().into_stream());
        if let Some(span) = span.into() {
            group.set_span(span);
        }
        Tokens::from_iter([TokenTree::Group(group)])
    }

    pub fn new_punct(char: char, last: bool, span: impl Into<Option<Span>>) -> Tokens {
        let mut punct = TokenTree::Punct(Punct::new(char, if last { Spacing::Alone } else { Spacing::Joint }));
        if let Some(span) = span.into() {
            punct.set_span(span);
        }
        Tokens::from_iter([punct])
    }

    pub fn new_punctuation(chars: impl AsRef<str>, span: impl Into<Option<Span>>) -> Tokens {
        let chars: Vec<char> = chars.as_ref().chars().collect();
        let span = span.into();
        let puncts = Vec::new();

        for (index, char) in chars.into_iter().enumerate() {
            let spacing = if index < chars.len() - 1 { Spacing::Joint } else { Spacing::Alone };
            let mut punct = TokenTree::Punct(Punct::new(char, spacing));
            if let Some(span) = span {
                punct.set_span(span);
            }
            puncts.push(punct);
        }

        Tokens::from_iter(puncts)
    }

    pub fn new_string_literal(string: impl AsRef<str>, span: impl Into<Option<Span>>) -> Tokens {
        let mut literal = Literal::string(string.as_ref());
        if let Some(span) = span.into() {
            literal.set_span(span);
        }
        Tokens::from_iter([TokenTree::Literal(literal)])
    }

    pub fn new_compile_error(msg: impl AsRef<str>, span: impl Into<Option<Span>>) -> Tokens {
        let ident = Tokens::new_ident("compile_error", span);
        let bang = Tokens::new_punctuation("!", span);
        let message = Tokens::new_string_literal(msg, span);
        let parens = Tokens::new_group('{', message, span);
        Tokens::from_iter([ident, bang, parens])
    }

    pub fn to_vec(&self) -> Vec<TokenTree> {
        self.clone().into_iter().collect()
    }

    pub fn to_stream(&self) -> TokenStream {
        self.stream.clone()
    }

    pub fn into_stream(self) -> TokenStream {
        self.stream
    }


    pub fn first(&self) -> Option<TokenTree> {
        self.to_vec().first().map(Clone::clone)
    }

    pub fn last(&self) -> Option<TokenTree> {
        self.to_vec().first().map(Clone::clone)
    }

    pub fn span(&self) -> Option<Span> {
        self.first().map(TokenTree::span)
    }

    pub fn first_span(&self) -> Option<Span> {
        self.first().map(|tt| match tt {
            TokenTree::Group(g) => g.span_open(),
            _ => tt.span()
        })
    }

    pub fn last_span(&self) -> Option<Span> {
        self.last().map(|tt| match tt {
            TokenTree::Group(g) => g.span_close(),
            _ => tt.span()
        })

    }

    pub fn to_error(&self, msg: impl AsRef<str>) -> Result<Tokens, Tokens> {
        Err(Tokens::new_compile_error(msg, self.first_span()))
    }

}

pub trait ToTokens {
    fn to_tokens(&self) -> Tokens;
}

pub trait IntoTokens: ToTokens + Sized {
    fn into_tokens(self) -> Tokens;
}

mod conversion_trait_impls {
    use crate::*;

    impl Default for Tokens {
        fn default() -> Self {
            Tokens { stream: TokenStream::new(), string: OnceCell::new() }
        }
    }


    impl ToTokens for Tokens {
        fn to_tokens(&self) -> Tokens {
            self.clone()
        }
    }

    impl IntoTokens for Tokens {
        fn into_tokens(self) -> Tokens {
            self
        }
    }

    impl IntoTokens for TokenStream {
        fn into_tokens(self) -> Tokens {
            Tokens {
                stream: self,
                ..Tokens::default()
            }
        }
    }

    impl ToTokens for TokenStream {
        fn to_tokens(&self) -> Tokens {
            self.clone().into_tokens()
        }
    }

    impl IntoTokens for TokenTree {
        fn into_tokens(self) -> Tokens {
            Tokens::from_iter([self])
        }
    }

    impl ToTokens for TokenTree {
        fn to_tokens(&self) -> Tokens {
            self.clone().into_tokens()
        }
    }

    impl IntoTokens for Group {
        fn into_tokens(self) -> Tokens {
            TokenTree::Group(self).into_tokens()
        }
    }

    impl ToTokens for Group {
        fn to_tokens(&self) -> Tokens {
            self.clone().into_tokens()
        }
    }

    impl IntoTokens for Punct {
        fn into_tokens(self) -> Tokens {
            TokenTree::Punct(self).into_tokens()
        }
    }

    impl ToTokens for Punct {
        fn to_tokens(&self) -> Tokens {
            self.clone().into_tokens()
        }
    }

    impl IntoTokens for Literal {
        fn into_tokens(self) -> Tokens {
            TokenTree::Literal(self).into_tokens()
        }
    }

    impl ToTokens for Literal {
        fn to_tokens(&self) -> Tokens {
            self.clone().into_tokens()
        }
    }

    impl IntoTokens for Ident {
        fn into_tokens(self) -> Tokens {
            TokenTree::Ident(self).into_tokens()
        }
    }

    impl ToTokens for Ident {
        fn to_tokens(&self) -> Tokens {
            self.clone().into_tokens()
        }
    }

    impl FromIterator<TokenTree> for Tokens {
        fn from_iter<I: IntoIterator<Item = TokenTree>>(iter: I) -> Self {
            TokenStream::from_iter(iter).into_tokens()
        }
    }

    impl FromIterator<Tokens> for Tokens {
        fn from_iter<I: IntoIterator<Item = Tokens>>(iter: I) -> Self {
            TokenStream::from_iter(iter.into_iter().flatten()).into_tokens()
        }
    }

    impl FromStr for Tokens {
        type Err = proc_macro::LexError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(TokenStream::from_str(s)?.into_tokens())
        }
    }

    impl IntoIterator for Tokens {
        type Item = TokenTree;
        type IntoIter = proc_macro::token_stream::IntoIter;

        fn into_iter(self) -> Self::IntoIter {
            self.into_stream().into_iter()
        }
    }

}



macro_rules! ignore {
    ($($tt:tt)*) => {}
}

ignore!{
    pub struct Tokens;
impl Tokens {


    pub fn from_stream(stream: impl Into<TokenStream>) -> Tokens {
        Tokens {
            first: OnceCell::new(),
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
        self.first = OnceCell::new();
        self.vec = OnceCell::new();
        self.string = OnceCell::new();
        self.stream.get_mut().unwrap()
    }

    pub fn into_stream(self) -> TokenStream {
        self.stream();
        self.stream.into_inner().unwrap()
    }

    pub fn from_vec(vec: Vec<TokenTree>) -> Tokens {
        Tokens {
            first: OnceCell::new(),
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

    pub fn from_tree(tree: TokenTree) -> Tokens {
        Tokens {
            first: OnceCell::with_value(Some(tree)),
            vec: OnceCell::new(),
            stream: OnceCell::new(),
            string: OnceCell::new(),
        }
    }

    pub fn tree(&self) -> Option<&TokenTree> {
        self.first
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
        self.first.get_mut().and_then(Option::as_mut)
    }

    pub fn into_tree(self) -> Option<TokenTree> {
        self.tree();
        self.first.into_inner().unwrap()
    }

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

    pub fn is_empty(&self) -> bool {
        if let Some(vec) = self.vec.get() {
            vec.is_empty()
        } else if let Some(stream) = self.stream.get() {
            stream.is_empty()
        } else if let Some(Some(_)) = self.first.get() {
            true
        } else {
            unreachable!()
        }
    }

    pub fn flat_map_shallow<F, I>(&self, f: F) -> Tokens
    where
        F: Fn(TokenTree) -> I,
        I: IntoIterator<Item = TokenTree>,
    {
        self.clone().into_iter().flat_map(f).collect()
    }

    pub fn flat_map_deep<F, I>(&self, f: F) -> Tokens
    where
        F: Fn(TokenTree) -> I,
        I: IntoIterator<Item = TokenTree>,
    {
        self.clone()
            .into_iter()
            .flat_map(f)
            .map(|tt| {
                if let TokenTree::Group(group) = tt {
                    let mut tt = TokenTree::Group(Group::new(
                        group.delimiter(),
                        group.stream().into_iter().flat_map(f).collect(),
                    ));
                    tt.set_span(group.span());
                    tt
                } else {
                    tt
                }
            })
            .collect()
    }

    pub fn with_span_shallow(&self, span: Span) -> Self {
        self.flat_map_shallow(|mut tree| {
            tree.set_span(span);
            Some(tree)
        })
    }

    pub fn with_span_deep(&self, span: Span) -> Self {
        self.flat_map_deep(|mut tree| {
            tree.set_span(span);
            Some(tree)
        })
    }

    pub fn replace_deep(&self, replacements: &HashMap<String, Tokens>) -> Tokens {
        self.flat_map_deep(|tt| match tt {
            TokenTree::Ident(ident) =>
                if let Some(replacement) = replacements.get(&ident.to_string()) {
                    replacement.clone()
                } else {
                    Tokens::from_tree(tt)
                },
            _ => Tokens::from_tree(tt),
        })
    }

    pub fn replace_shallow(&self, replacements: &HashMap<String, Tokens>) -> Tokens {
        self.flat_map_shallow(|tt| match tt {
            TokenTree::Ident(ident) =>
                if let Some(replacement) = replacements.get(&ident.to_string()) {
                    replacement.clone()
                } else {
                    Tokens::from_tree(tt)
                },
            _ => Tokens::from_tree(tt),
        })
    }

    pub fn len(&self) -> usize {
        self.vec().len()
    }

    pub fn first(&self) -> Result<&TokenTree, TokenTree> {
        self.vec().first().ok_or_else(|| {})
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
        if let Some(Some(_)) = self.first.get() {
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

impl FromIterator<Tokens> for Tokens {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Tokens>,
    {
        let mut vec = Vec::new();
        for tokens in iter {
            vec.extend(tokens.into_iter());
        }
        Tokens::from_vec(vec)
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

impl From<Result<Tokens, Tokens>> for Tokens {
    fn from(literal: Result<Tokens, Tokens>) -> Self {
        match literal {
            Ok(tokens) => tokens,
            Err(tokens) => tokens,
        }
    }
}

impl Tokens {
    pub fn extend(&mut self, rhs: Tokens) {
        if self.stream.get().is_none() && self.vec.get().is_none() {
            self.vec();
        }

        self.string = OnceCell::new();
        self.first = OnceCell::new();

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

pub type ResultTokens = Result<Tokens, Tokens>;

pub trait ToTokens {
    fn to_tokens(&self) -> Tokens;

    fn span(&self) -> ResultTokens {
        let span = self.to_tokens().first()?.span().unwrap_or_else(Span::call_site);

        Ok(todo!())
    }
}

pub trait IntoTokens: ToTokens + Sized {
    fn into_tokens(self) -> Tokens;

    fn error<M: AsRef<str>, R>(self, message: M) -> Result<R, Tokens> {
        Err(Tokens::error(&self.into_tokens(), message))
    }
}
