pub trait ExtTapPipe {
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
}

impl<T> ExtTapPipe for T {}
