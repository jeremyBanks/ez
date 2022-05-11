use crate::*;

pub struct MetaBrush<Inner: Brush, Behavior: MetaBrushBehavior> {
    inner: Inner,
    behavior: Behavior,
}

impl<Inner: Brush, Behavior: MetaBrushBehavior> MetaBrush<Inner, Behavior> {
    pub fn new(behavior: Behavior, inner: Inner) -> Self {
        Self { inner, behavior }
    }

    pub fn end(self) -> Inner {
        self.inner
    }
}

pub trait MetaBrushBehavior {
    fn stroke(&mut self, inner: &mut impl Brush, distance: Ratio) {
        inner.stroke(distance)
    }

    fn rotate(&mut self, inner: &mut impl Brush, revolutions: Revolutions) {
        inner.rotate(revolutions)
    }
}

impl<Inner: Brush, Behavior: MetaBrushBehavior> Brush for MetaBrush<Inner, Behavior> {
    fn stroke(&mut self, distance: Ratio) {
        self.behavior.stroke(&mut self.inner, distance)
    }

    fn rotate(&mut self, revolutions: Revolutions) {
        self.behavior.rotate(&mut self.inner, revolutions)
    }
}
