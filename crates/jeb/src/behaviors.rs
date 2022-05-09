use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct Mirrored;
impl MetaBrushBehavior for Mirrored {
    fn rotate(inner: &mut impl Brush, revolutions: Revolutions) {
        inner.rotate(-revolutions);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Scaled<const RATIO: Ratio>;
impl<const RATIO: Ratio> MetaBrushBehavior for Scaled<RATIO> {
    fn stroke(inner: &mut impl Brush, length: Ratio) {
        inner.stroke(RATIO * length);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ZigZag;
impl MetaBrushBehavior for ZigZag {
    fn stroke(inner: &mut impl Brush, distance: Ratio) {
        inner.rotate(revolutions(-0.25));
        inner.stroke(SQRT_2 / 4.0 * distance);
        inner.rotate(revolutions(0.50));
        inner.stroke(SQRT_2 / 2.0 * distance);
        inner.rotate(revolutions(-0.50));
        inner.stroke(SQRT_2 / 4.0 * distance);
        inner.rotate(revolutions(0.25));
    }
}
