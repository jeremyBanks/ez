use crate::*;

/// Extension method for all values; a subset of the `tap` crate.
pub trait AllExt {
    fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R
    where
        Self: Sized,
    {
        f(self)
    }

    fn pipe_ref<R>(&self, f: impl FnOnce(&Self) -> R) -> R {
        f(self)
    }

    fn pipe_mut<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        f(self)
    }

    fn tap(self, f: impl FnOnce(&Self) -> ()) -> Self
    where
        Self: Sized,
    {
        f(&self);
        self
    }

    fn tap_mut(mut self, f: impl FnOnce(&mut Self) -> ()) -> Self
    where
        Self: Sized,
    {
        f(&mut self);
        self
    }

    fn conv<T>(self) -> T
    where
        Self: Sized + Into<T>,
    {
        self.into()
    }

    fn try_conv<T, E>(self) -> Result<T, E>
    where
        Self: Sized + TryInto<T, Error = E>,
    {
        self.try_into()
    }
}

impl<T> AllExt for T {}
