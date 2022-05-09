use crate::*;

pub trait Design {
    const X: Ratio = 0.into();
    const Y: Ratio = 0.into();
    const ORIENTATION: Revolutions = 0.into();

    fn draw<SomeBrush: Brush>(brush: &mut SomeBrush, scale: Ratio);
}

pub struct Square;
impl Design for Square {
    const X: Ratio = Ratio::new(0.25);
    const Y: Ratio = Ratio::new(0.25);

    fn draw<SomeBrush: Brush>(brush: &mut SomeBrush, scale: Ratio) {
        brush.right_turn(0.5 * scale);
        brush.right_turn(0.5 * scale);
        brush.right_turn(0.5 * scale);
        brush.right_turn(0.5 * scale);
    }
}
