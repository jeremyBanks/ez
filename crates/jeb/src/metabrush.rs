use crate::*;

pub struct MetaBrush<'inner, Behavior: MetaBrushBehavior, Inner: Brush> {
    inner: &'inner mut Inner,
    behavior: Behavior,
}

impl<'inner, Behavior: MetaBrushBehavior, Inner: Brush> MetaBrush<'inner, Behavior, Inner> {
    pub fn new(behavior: Behavior, inner: &'inner mut Inner) -> Self {
        Self { inner, behavior }
    }

    pub fn end<'outer>(&'outer mut self) -> &'outer mut Inner {
        self.inner
    }
}

pub trait MetaBrushBehavior {
    fn stroke<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, scale: Ratio) {
        inner.stroke(scale);
    }

    fn rotate<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, revolutions: Revolutions) {
        inner.rotate(revolutions);
    }

    fn left_turn<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, scale: Ratio) {
        inner.left_turn(scale);
    }
}

impl<Behavior: MetaBrushBehavior, Inner: Brush> Brush for MetaBrush<'_, Behavior, Inner> {
    fn stroke(&mut self, distance: Ratio) -> &mut Self {
        self.behavior.stroke(self.inner, distance);
        self
    }

    fn rotate(&mut self, revolutions: Revolutions) -> &mut Self {
        self.behavior.rotate(self.inner, revolutions);
        self
    }

    fn left_turn(&mut self, distance: Ratio) -> &mut Self {
        self.behavior.left_turn(self.inner, distance);
        self
    }
}
