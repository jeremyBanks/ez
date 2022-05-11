use crate::*;

pub trait Brush: Sized {
    fn stroke(&mut self, scale: Ratio) -> &mut Self;

    fn rotate(&mut self, revolutions: Revolutions) -> &mut Self;

    fn rotate_left(&mut self, revolutions: Revolutions) -> &mut Self {
        self.rotate(revolutions)
    }

    fn rotate_right(&mut self, revolutions: Revolutions) -> &mut Self {
        self.mirrored().rotate_left(revolutions);
        self
    }

    fn left_turn(&mut self, scale: Ratio) -> &mut Self {
        self.stroke(0.5 * scale);
        self.rotate(0.25);
        self.stroke(0.5 * scale);
        self
    }

    fn right_turn(&mut self, scale: Ratio) -> &mut Self {
        self.mirrored().left_turn(scale);
        self
    }

    fn left_loop<'this>(&'this mut self, scale: Ratio) -> &mut Self {
        self.left_turn(scale).left_turn(scale).left_turn(scale).left_turn(scale)
    }

    fn right_loop<'this>(&'this mut self, scale: Ratio) -> &mut Self {
        self.mirrored().left_loop(scale);
        self
    }

    fn with<'this, Behavior: MetaBrushBehavior + Sized>(
        &'this mut self,
        behaviour: Behavior,
    ) -> MetaBrush<Behavior, Self> {
        MetaBrush::new(behaviour, self)
    }

    fn scaled<'this>(&'this mut self, scale: Ratio) -> MetaBrush<Scaled, Self> {
        self.with(Scaled(scale))
    }

    fn mirrored<'this>(&'this mut self) -> MetaBrush<Mirrored, Self> {
        self.with(Mirrored)
    }

    fn zig_zag<'this>(&'this mut self) -> MetaBrush<ZigZag, Self> {
        self.with(ZigZag)
    }

    fn sharp_turns<'this>(&'this mut self) -> MetaBrush<SharpTurns, Self> {
        self.with(SharpTurns)
    }
}
