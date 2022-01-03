use ::core::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Int(i128);

impl Add<Int> for usize {
    type Output = usize;
    fn add(self, other: Int) -> usize {
        let other: usize = other.0.try_into().unwrap();
        self.add(other)
    }
}

impl Add<usize> for Int {
    type Output = usize;
    fn add(self, other: usize) -> usize {
        let self_: usize = self.0.try_into().unwrap();
        self_.add(other)
    }
}

impl Add<Int> for Int {
    type Output = Int;
    fn add(self, other: Int) -> Int {
        Self(self.0.add(other.0))
    }
}

impl<T> Index<Int> for [T] {
    type Output = T;
    fn index(&self, index: Int) -> &T {
        // What if it's negative? No problem?
        // Do like the pythonistas.
        let index = index.0 as usize;
        Index::<usize>::index(self, index)
    }
}

impl<T> Index<Int> for Vec<T> {
    type Output = T;
    fn index(&self, index: Int) -> &T {
        let index = index.0 as usize;
        Index::<usize>::index(self, index)
    }
}

fn main() {
    let i32: i32 = 0;

    let n = Int(0);
    let v = vec![1, 2, 3];

    dbg!(v[0]);
    dbg!(&v[0 + n]);
    dbg!(&v[n + 0]);
    dbg!(&v[n]);

    let a = [1, 2, 3];

    dbg!(a[0]);
    dbg!(&a[0 + n]);
    dbg!(&a[n + 0]);
    dbg!(&a[n]);

    let s = &a;

    dbg!(s[0]);
    dbg!(&s[0 + n]);
    dbg!(&s[n + 0]);
    dbg!(&s[n]);

    let s = &v[..];

    dbg!(s[0]);
    dbg!(&s[0 + n]);
    dbg!(&s[n + 0]);
    dbg!(&s[n]);
}
