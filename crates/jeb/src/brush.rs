use crate::*;

pub trait Brush {
    /// Extends the path in the current direction by the given distance.
    fn stroke(&mut self, distance: Ratio);
    /// Rotates the orientation by the given number of revolutions.
    fn rotate(&mut self, revolutions: Revolutions);

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

    fn sharp_turns(self) -> MetaBrush<Self, SharpTurns> {
        self.with(SharpTurns)
    }

    fn round_turns(self) -> MetaBrush<Self, RoundTurns> {
        self.with(RoundTurns)
    }

    fn rounded_turns(self, roundness: Ratio) -> MetaBrush<Self, RoundedTurns> {
        self.with(RoundedTurns(roundness))
    }

    fn zig_zag_strokes(self) -> MetaBrush<Self, ZigZagStrokes> {
        self.with(ZigZagStrokes)
    }

    fn wavy_strokes(self) -> MetaBrush<Self, WavyStrokes> {
        self.with(WavyStrokes)
    }
}
