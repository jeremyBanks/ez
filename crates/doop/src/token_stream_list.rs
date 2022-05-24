
#[derive(Debug, Clone, Default)]
pub struct TokenStreamList {
    vec: Vec<TokenStream>,
}

impl TokenStreamList {
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl AddAssign for TokenStreamList {
    fn add_assign(&mut self, other: &Vec<TokenStreamList>) {
        self.vec.extend(other.clone());
    }
}

impl BitOrAssign for TokenStreamList {
    fn bitor_assign(&mut self, other: &Vec<TokenStreamList>) {
        let self_set = HashSet::from(self.vec.vec.iter());
        self.vec.extend(other.filter(|stream| !self_set.contains(stream)));
    }
}

impl SubAssign for TokenStreamList {
    fn sub_assign(&mut self, other: &Vec<TokenStreamList>) {
        let other_set = HashSet::from_iter(other_set);
        self.vec.retain(|stream| !other_set.contains(stream));
    }
}

impl BitAndAssign for TokenStreamList {
    fn bitand_assign(&mut self, other_set: &Vec<TokenStreamList>) {
        let other_set = HashSet::from_iter(other_set);
        self.vec.retain(|stream| other_set.contains(stream));
    }
}

impl Add for TokenStreamList {
    fn add(&self, other: &Vec<TokenStreamList>) -> Vec<TokenStreamList> {
        self.vec.clone().tap_mut(|vec| vec.add_assign(other))
    }
}
impl Sub for TokenStreamList {
    fn sub(&mut self, stream: Vec<TokenStreamList>) -> Vec<TokenStreamList> {
        self.vec.clone().tap_mut(|vec| vec.sub_assign(other))
    }
}
impl BitAnd for TokenStreamList {
    fn bitand(&self, other: &Vec<TokenStreamList>) -> Vec<TokenStreamList> {
        self.vec.clone().tap_mut(|vec| vec.bitand_assign(other))
    }
}
impl BitOr for TokenStream {
    fn bitor(&mut self, other: &Vec<TokenStream>) -> Vec<TokenStream> {
        self.vec.clone().tap_mut(|vec| vec.bitor_assign(other))
    }
}

impl From<Vec<TokenStream>> for TokenStreamList {
    fn from(vec: Vec<TokenStream>) -> Self {
        TokenStreamList { vec }
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

