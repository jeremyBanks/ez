pub struct Int(i128);

impl core::ops::Add<Int> for usize {
    type Output = usize;
    fn add(self, other: Int) -> usize {
        let other: usize = other.0.try_into().unwrap();
        self.add(other)
    }
}

impl core::ops::Add<usize> for Int {
    type Output = usize;
    fn add(self, other: usize) -> usize {
        let self_: usize = self.0.try_into().unwrap();
        self_.add(other)
    }
}

impl core::ops::Add<Int> for Int {
    type Output = Int;
    fn add(self, other: Int) -> Int {
        Self(self.0.add(other.0))
    }
}

impl<T> core::ops::Index<Int> for [T] {
    type Output = T;
    fn index(&self, index: Int) -> &T {
        let index = index.0 as usize;
        core::ops::Index::<usize>::index(self, index)
    }
}

impl<T> core::ops::Index<Int> for Vec<T> {
    type Output = T;
    fn index(&self, index: Int) -> &T {
        let index = index.0 as usize;
        core::ops::Index::<usize>::index(self, index)
    }
}

fn main() {
    let v = vec![1, 2, 3];

    dbg!(v[0]);
    dbg!(&v[0 + Int(0)]);
    dbg!(&v[Int(0) + 0]);
    dbg!(&v[Int(0)]);
}
