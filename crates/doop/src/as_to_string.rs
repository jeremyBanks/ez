use {
    once_cell::unsync::OnceCell,
    std::{
        cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
        fmt::{Debug, Display},
        hash::Hash,
        ops::{Deref, DerefMut},
    },
};

// This should be AsRef<str> not CompareAsStr.

// Two types:
// One that adds an AsRef<str> and a Format<String> to a type, using a memoized
// to_string on the underlying type.
// This is non-zero-sized, of course.

// One that takes an AsRef<str> type and uses it for all comparison operators
// This is a zero-sized wrapper.

pub fn main() {
    let mut numbers = Vec::from_iter(b"Hello, world!".into_iter().copied());
}

pub fn like_string<T: Display>(t: T) -> CompareAsStr<AsLazyStr<T>> {
    CompareAsStr::wrap(AsLazyStr::wrap(t))
}

pub fn unlike_string<T: Display>(t: CompareAsStr<AsLazyStr<T>>) -> T {
    t.unwrap().unwrap()
}

/// Wrapper type implementing AsRef<str> and Display, memoizing the inner
/// value's to_string().
pub struct AsLazyStr<Inner: Display> {
    inner: Inner,
    string: OnceCell<String>,
}

impl<Inner: Display> AsLazyStr<Inner> {
    pub fn wrap(inner: Inner) -> AsLazyStr<Inner> {
        AsLazyStr { inner, string: OnceCell::new() }
    }

    pub fn unwrap(self) -> Inner {
        self.inner
    }
}

impl<Inner: Display> Deref for AsLazyStr<Inner> {
    type Target = Inner;

    fn deref(&self) -> &Inner {
        &self.inner
    }
}

impl<Inner: Display> AsRef<str> for AsLazyStr<Inner> {
    fn as_ref(&self) -> &str {
        self.string.get_or_init(|| self.inner.to_string())
    }
}

impl<Inner: Display> Display for AsLazyStr<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_ref(), f)
    }
}

impl<Inner: Display + Debug> Debug for AsLazyStr<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

/// Wrapper newtype implementing comparison operators `Eq`, `Ord`, `PartialEq`,
/// PartialOrd`, and `Hash` for the inner type based on the result of its
/// `AsRef<str>`.
pub struct CompareAsStr<Inner: AsRef<str>>(Inner);

impl<Inner: AsRef<str>> CompareAsStr<Inner> {
    pub fn wrap(inner: Inner) -> CompareAsStr<Inner> {
        CompareAsStr(inner)
    }

    pub fn unwrap(self) -> Inner {
        self.0
    }
}

impl<Inner: AsRef<str>> Deref for CompareAsStr<Inner> {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Inner: AsRef<str>> Display for CompareAsStr<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_ref(), f)
    }
}

impl<Inner: AsRef<str> + Debug> Debug for CompareAsStr<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<Inner: AsRef<str>> Ord for CompareAsStr<Inner> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

impl<Inner: AsRef<str>> PartialOrd for CompareAsStr<Inner> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Inner: AsRef<str>> PartialEq for CompareAsStr<Inner> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<Inner: AsRef<str>> Eq for CompareAsStr<Inner> {}

impl<Inner: AsRef<str>> Hash for CompareAsStr<Inner> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}
