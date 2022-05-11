use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct Mirrored;
impl MetaBrushBehavior for Mirrored {
    fn rotate<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, revolutions: Revolutions) {
        inner.rotate(-revolutions);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Scaled(pub Ratio);
impl MetaBrushBehavior for Scaled {
    fn stroke<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, scale: Ratio) {
        inner.stroke(scale * self.0);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ZigZag;
impl MetaBrushBehavior for ZigZag {
    fn stroke<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, scale: Ratio) {
        inner.rotate(-0.25);
        inner.stroke(SQRT_2 / 4.0 * scale);
        inner.rotate(0.50);
        inner.stroke(SQRT_2 / 2.0 * scale);
        inner.rotate(-0.50);
        inner.stroke(SQRT_2 / 4.0 * scale);
        inner.rotate(0.25);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SharpTurns;
impl MetaBrushBehavior for SharpTurns {
    fn left_turn<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, scale: Ratio) {
        inner.stroke(0.5 * scale).rotate(0.25).stroke(0.5 * scale);
    }
}
