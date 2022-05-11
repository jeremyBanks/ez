use crate::*;

/// Rederives all default brush method implementations from the inner's `stroke` and `rotate`.
#[derive(Debug, Clone, Copy)]
pub struct Defaults;
impl MetaBrushBehavior for Defaults {
    fn stroke<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, scale: f64) {
        inner.stroke(scale);
    }

    fn rotate<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, revolutions: f64) {
        inner.rotate(revolutions);
    }

    fn rotate_left<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, revolutions: f64) {
        self.rotate(inner, revolutions);
    }

    fn rotate_right<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, revolutions: f64) {
        self.rotate(inner, -revolutions);
    }

    fn rotate_left<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, scale: f64) {
        self.stroke(0.5 * scale);
        self.rotate(0.25);
        self.stroke(0.5 * scale);
        self
    }
}



#[derive(Clone, Copy, Debug)]
pub struct Mirrored;
impl MetaBrushBehavior for Mirrored {
    fn rotate<'inner, Inner: Brush>(&mut self, inner: &'inner mut Inner, revolutions: Revolutions) {
        inner.rotate(-revolutions);
    }

    // XXX: this needs to manually flip all of the things, I'm afraid
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


#[derive(Clone, Copy, Debug)]
pub struct TurnLoops;
impl MetaBrushBehavior for TurnLoops {

}
