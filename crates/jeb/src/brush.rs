use std::fmt::Debug;

use crate::*;

pub trait Brush: Sized {
    fn stroke(&mut self, scale: Ratio) -> &mut Self;

    fn rotate(&mut self, revolutions: Revolutions) -> &mut Self;

    fn rotate_left(&mut self, revolutions: Revolutions) -> &mut Self {
        self.defaults().rotate_left(revolutions);
        self
    }

    fn rotate_right(&mut self, revolutions: Revolutions) -> &mut Self {
        self.defaults().rotate_right(revolutions);
        self
    }

    fn left_turn(&mut self, scale: Ratio) -> &mut Self {
        self.defaults().left_turn(scale);
        self
    }

    fn right_turn(&mut self, scale: Ratio) -> &mut Self {
        self.defaults().right_turn(scale);
        self
    }

    fn left_loop(&mut self, scale: Ratio) -> &mut Self {
        self.defaults().left_loop();
    }

    fn right_loop(&mut self, scale: Ratio) -> &mut Self {
        self.defaults().right_loop();
    }

    fn fill(&mut self, width: Ratio, height: Ratio) -> &mut Self {
        self.defaults().fill(width, height);
    }

    fn scaled(&mut self, scale: Ratio) -> MetaBrush<Scaled, Self> {
        MetaBrush::new(Scaled(scale), self)
    }

    fn mirrored(&mut self) -> MetaBrush<Mirrored, Self> {
        MetaBrush::new(Mirrored, self)    }

    fn zig_zag(&mut self) -> MetaBrush<ZigZag, Self> {
        MetaBrush::new(ZigZag, self)
    }

    fn sharp_turns(&mut self) -> MetaBrush<SharpTurns, Self> {
        MetaBrush::new(SharpTurns, self)
    }

    fn turn_loops(&mut self) -> MetaBrush<SharpTurns, Self> {
        MetaBrush::new(TurnLoops, self)
    }

    /// default brush layers to implement optional methods from required ones
    fn defaults(&mut self) -> MetaBrush<TurnLoops, MetaBrush<SharpTurns, Self>> {
        self.sharp_turns().turn_loops()
    }
}

pub trait MetaBrushBehavior: Debug + Copy + Clone {
    fn stroke<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, scale: Ratio) {
        inner.stroke(scale);
    }
}
