pub trait WeaklyFrom<T>: Sized {
    type Error: std::error::Error;
    fn weakly_from(x: T) -> Self {
        Self::try_weakly_from(x).unwrap()
    }
    fn try_weakly_from(x: T) -> Result<Self, Self::Error>;
}
pub trait WeaklyInto<T> {
    fn weakly_into(self) -> T;
}

impl<T, U> WeaklyInto<U> for T
where
    U: WeaklyFrom<T>,
{
    fn weakly_into(self) -> U {
        U::weakly_from(self)
    }
}
