use crate::*;

pub struct MetaBrush<Brush: crate::brush::Brush, Behavior: MetaBrushBehavior = NoBehavior> {
    brush: Brush,
    behavior: Behavior,
}

pub trait MetaBrushBehavior {
    fn stroke(&mut self, inner: &mut impl crate::brush::Brush, distance: Ratio) {
        inner.stroke(distance)
    }

    fn rotate(&mut self, inner: &mut impl crate::brush::Brush, revolutions: Revolutions) {
        inner.rotate(revolutions)
    }
}

// impl<Brush: crate::brush::Brush, Behavior: MetaBrushBehavior>
// crate::brush::Brush for MetaBrush<Brush, Behavior> {     fn stroke(&mut self,
// distance: Ratio) {         self.behavior.stroke(&mut self.brush, distance)
//     }

//     fn rotate(&mut self, revolutions: Revolutions) {
//         self.behavior.rotate(&mut self.brush, revolutions)
//     }
// }
