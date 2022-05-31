use {
    once_cell::unsync::OnceCell,
    std::{
        cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
        fmt::{Debug, Display},
        hash::Hash,
        ops::{Deref, DerefMut},
    },
};

pub fn main() {
    let mut numbers = vec![20, 22, 23, 24, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    println!("original {numbers:?}");
    numbers.sort();
    println!("sorted {numbers:?}");
    let mut numbers: Vec<_> = numbers.iter().map(AsToString).collect();
    numbers.sort();
    println!("sorted as strings {numbers:?}");
    let mut numbers: Vec<_> = numbers.into_iter().map(AsToString::unwrap).collect();
    numbers.sort();
}

pub fn like_string<Inner: Display + Debug>(inner: Inner) -> AsToString<SavedToString<Inner>> {
    AsToString::wrap(SavedToString::wrap(inner))
}

pub fn unlike_string<Inner: Display + Debug>(wrapped: AsToString<SavedToString<Inner>>) -> Inner {
    wrapped.unwrap().unwrap()
}

pub struct AsToString<Inner: Display + Debug>(Inner);

impl<Inner: Display + Debug> AsToString<Inner> {
    pub fn wrap(inner: Inner) -> AsToString<Inner> {
        AsToString(inner)
    }

    pub fn unwrap(self) -> Inner {
        self.0
    }
}

pub struct SavedToString<Inner: Display + Debug> {
    inner: Inner,
    string: OnceCell<String>,
}

impl<Inner: Display + Debug> SavedToString<Inner> {
    pub fn wrap(inner: Inner) -> SavedToString<Inner> {
        SavedToString { inner, string: OnceCell::new() }
    }

    pub fn unwrap(self) -> Inner {
        self.inner
    }

    pub fn as_str(&self) -> &str {
        self.string.get_or_init(|| self.inner.to_string()).as_str()
    }
}

impl<Inner: Display + Debug> Display for SavedToString<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl<Inner: Display + Debug> Debug for SavedToString<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<Inner: Display + Debug> Display for AsToString<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<Inner: Display + Debug> Debug for AsToString<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<Inner: Display + Debug> Ord for AsToString<Inner> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.0.to_string())
    }
}

impl<Inner: Display + Debug> PartialOrd for AsToString<Inner> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Inner: Display + Debug> PartialEq for AsToString<Inner> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.0.to_string()
    }
}

impl<Inner: Display + Debug> Eq for AsToString<Inner> {}

impl<Inner: Display + Debug> Hash for AsToString<Inner> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

impl<Inner: Display + Debug> Deref for AsToString<Inner> {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
