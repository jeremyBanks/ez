use crate::*;

pub trait Brush: Sized {
    /// Extends the path in the current direction by the given distance.
    fn stroke(&mut self, distance: Ratio);

    /// Rotates the orientation by the given number of revolutions.
    fn rotate(&mut self, revolutions: Revolutions);

    fn right_turn(&mut self, distance: Ratio) {
        self.stroke(0.5 * distance);
        self.rotate(revolutions(0.25));
        self.stroke(0.5 * distance);
    }

    fn left_turn(&mut self, distance: Ratio) {
        self.mirrored().right_turn(distance);
    }

    fn with<Behavior: MetaBrushBehavior + Sized>(
        self,
        behaviour: Behavior,
    ) -> MetaBrush<Self, Behavior> {
        MetaBrush::new(behaviour, self)
    }

    fn scaled<const RATIO: Ratio>(self) -> MetaBrush<Self, Scaled<RATIO>> {
        self.with(Scaled)
    }

    fn mirrored(self) -> MetaBrush<Self, Mirrored> {
        self.with(Mirrored)
    }
}
