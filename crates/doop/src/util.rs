pub trait Pipe {
    fn tap_mut(&mut self, f: impl FnOnce(&mut Self) -> ()) {
        f(self)
    }
}

impl<T> Pipe for T {}
