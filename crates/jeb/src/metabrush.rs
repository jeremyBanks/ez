use std::marker::PhantomData;

use crate::*;

pub struct MetaBrush<Inner: Brush, Behavior: MetaBrushBehavior> {
    inner: Inner,
    behavior: PhantomData<Behavior>,
}

impl<Inner: Brush, Behavior: MetaBrushBehavior> MetaBrush<Inner, Behavior> {
    pub fn new(behavior: Behavior, inner: Inner) -> Self {
        Self { inner, behavior: PhantomData }
    }
}

pub trait MetaBrushBehavior {
    fn stroke(inner: &mut impl Brush, distance: Ratio) {
        inner.stroke(distance)
    }

    fn rotate(inner: &mut impl Brush, revolutions: Revolutions) {
        inner.rotate(revolutions)
    }
}

impl<Inner: Brush, Behavior: MetaBrushBehavior> Brush for MetaBrush<Inner, Behavior> {
    fn stroke(&mut self, distance: Ratio) {
        Behavior::stroke(&mut self.inner, distance)
    }

    fn rotate(&mut self, revolutions: Revolutions) {
        Behavior::rotate(&mut self.inner, revolutions)
    }
}
