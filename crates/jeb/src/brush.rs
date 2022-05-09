use crate::*;

pub trait Brush {
    /// Extends the path in the current direction by the given distance.
    fn stroke(&mut self, distance: Ratio);
    /// Rotates the orientation by the given number of revolutions.
    fn rotate(&mut self, revolutions: Revolutions);

    fn right_turn(&mut self, distance: Ratio) {
        self.stroke(0.5 * distance);
        self.rotate(Revolutions::new(0.25));
        self.stroke(0.5 * distance);
    }

    fn left_turn(&mut self, distance: Ratio) {
        self.mirrored().right_turn(distance);
    }

    fn with<Behavior: MetaBrushBehavior + Sized>(
        self,
        behaviour: Behavior,
    ) -> MetaBrush<Self, Behavior>
    where
        Self: Sized,
    {
        MetaBrush::new(behaviour, self)
    }

    fn scaled(self, scale: Ratio) -> MetaBrush<Self, Scaled> {
        self.with(Scaled(scale))
    }

    fn mirrored(self) -> MetaBrush<Self, Mirrored> {
        self.with(Mirrored)
    }
}
