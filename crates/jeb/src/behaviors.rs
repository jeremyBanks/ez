use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct Mirrored;
impl MetaBrushBehavior for Mirrored {
    fn rotate(&mut self, inner: &mut impl Brush, revolutions: Revolutions) {
        inner.rotate(-revolutions);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Scaled(pub Ratio);
impl MetaBrushBehavior for Scaled {
    fn stroke(&mut self, inner: &mut impl Brush, distance: Ratio) {
        inner.stroke(distance * self.0);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ZigZag;
impl MetaBrushBehavior for ZigZag {
    fn stroke(&mut self, inner: &mut impl Brush, distance: Ratio) {
        inner.rotate(revolutions(-0.25));
        inner.stroke(SQRT_2 / 4.0 * distance);
        inner.rotate(revolutions(0.50));
        inner.stroke(SQRT_2 / 2.0 * distance);
        inner.rotate(revolutions(-0.50));
        inner.stroke(SQRT_2 / 4.0 * distance);
        inner.rotate(revolutions(0.25));
    }
}
