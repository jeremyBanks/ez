use crate::*;

pub trait Drawing: Sized {
    fn draw(brush: &mut impl Brush, scale: Ratio);
}

pub trait Filling: Drawing {
    fn fill(brush: &mut impl Brush, width: Ratio, height: Ratio) {
        Self::draw(brush, width.min(height))
    }
}

pub trait Root: Drawing {
    const X: Ratio = 0.5.into();
    const Y: Ratio = Self::X;
    const ORIENTATION: Revolutions = revolutions(0.0);
}

#[derive(Debug, Clone, Copy)]
pub struct FourSquare;

impl Drawing for FourSquare {
    fn draw(brush: &mut impl Brush, scale: Ratio) {
        brush.right_turn(0.5 * scale);
        brush.right_turn(0.5 * scale);
        brush.right_turn(0.5 * scale);
        brush.right_turn(0.5 * scale);
    }
}

impl Root for FourSquare {
    const X: Ratio = ratio(0.25);
    const Y: Ratio = ratio(0.25);
}
