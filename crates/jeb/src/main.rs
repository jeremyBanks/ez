mod behaviors;
mod brush;
mod design;
mod metabrush;
mod svg;
mod units;

pub use crate::{behaviors::*, brush::*, design::*, metabrush::*, svg::*, units::*};

fn main() {
    let mut svg = SVGPath::default();

    svg.scaled(0.75)
        .right_loop(0.75)
        .end()
        .scaled(0.5)
        .mirrored()
        .right_loop(0.5)
        .left_loop(0.25)
        .end()
        .end();

    // zig_zag.move_to(0.5, 0.5);

    // jeb(&mut zig_zag, 0.25);

    // zig_zag.stroke(0.25);
    // zig_zag.rotate_left(0.25);
    // zig_zag.stroke(0.25);
    // zig_zag.rotate_right(0.50);
    // zig_zag.stroke(0.50);

    // let svg = brush::MetaBrush::take(zig_zag);

    // let doc = templates::document(&format!("<path d=\"\n{}\n\" />",
    // "svg.path()"));

    // println!("{doc}");
}
use std::{
    f64::consts::{SQRT_2, TAU},
    ops::{Deref, DerefMut},
    rc::Rc,
};

// #[derive(Debug, Clone)]
// struct ZigZagBrush<Inner: Brush> {
//     inner: Inner,
// }

// impl<Inner: Brush> brush::MetaBrush for ZigZagBrush<Inner> {
//     type Inner = Inner;

//     fn take(self) -> Inner {
//         self.inner
//     }
// }

// // impl<Inner: Brush> Brush for ZigZagBrush<Inner> {
// //     fn stroke(&mut self, width: f64) {
// //         self.rotate(0.25);
// //         self.inner.stroke(width);
// //     }
// // }

// impl<Inner: Brush> From<Inner> for ZigZagBrush<Inner> {
//     fn from(inner: Inner) -> Self {
//         Self { inner }
//     }
// }

// impl<Inner: Brush> Deref for ZigZagBrush<Inner> {
//     type Target = Inner;
//     fn deref(&self) -> &Inner {
//         &self.inner
//     }
// }

// impl<Inner: Brush> DerefMut for ZigZagBrush<Inner> {
//     fn deref_mut(&mut self) -> &mut Inner {
//         &mut self.inner
//     }
// }

// use patterns::*;
// mod patterns {
//     use super::brush::Brush;

//     pub type Pattern = fn(brush: &mut dyn Brush, scale: f64) -> ();

//     pub fn jeb(brush: &mut dyn Brush, scale: f64) {
//         fn right(brush: &mut dyn Brush, scale: f64) {
//             brush.right_arm(scale);
//         }
//         fn left(brush: &mut dyn Brush, scale: f64) {
//             brush.left_arm(scale);
//         }
//         fn straight(brush: &mut dyn Brush, scale: f64) {
//             brush.stroke(scale);
//         }
//         let steps = &[
//             right, right, left, left, right, right, left, right, left, right, left, right,
//             straight, right, right, straight, left, left, straight, left, right, right,
// left,             right, left, right, right, left, left, right, right, straight, straight,
// right, right,             left, right, left, left, straight, straight, straight, straight,
// straight, left, left,             right, left, right, right, straight, straight,
//         ];
//         for step in steps {
//             step(brush, scale);
//         }
//     }
// }

// use brush::{Brush, SVGPath};
// mod brush {
//     use std::{
//         f64::consts::{SQRT_2, TAU},
//         ops::{Deref, DerefMut},
//     };

//     pub trait Brush {
//         fn move_to(&mut self, x: f64, y: f64);

//         fn rotate(&mut self, turns_clockwise: f64);

//         fn stroke(&mut self, scale: f64);

//         fn rotate_right(&mut self, turns: f64) {
//             <Self as Brush>::rotate(self, turns)
//         }

//         fn rotate_left(&mut self, turns: f64) {
//             <Self as Brush>::rotate(self, -turns)
//         }

//         fn left_turn(&mut self, scale: f64) {}

//         fn left_arm(&mut self, scale: f64) {
//             <Self as Brush>::stroke(self, 0.500 * scale);
//             <Self as Brush>::rotate_left(self, 0.250);
//             <Self as Brush>::stroke(self, 0.500 * scale);
//         }

//         fn left_curve(&mut self, scale: f64) {
//             <Self as Brush>::rotate_left(self, 0.125);
//             <Self as Brush>::stroke(self, SQRT_2 / 2.000 * scale);
//             <Self as Brush>::rotate_left(self, 0.125);
//         }

//         fn right_arm(&mut self, scale: f64) {
//             <Self as Brush>::stroke(self, 0.500 * scale);
//             <Self as Brush>::rotate_right(self, 0.250);
//             <Self as Brush>::stroke(self, 0.500 * scale);
//         }

//         fn right_curve(&mut self, scale: f64) {
//             <Self as Brush>::rotate_right(self, 0.125);
//             <Self as Brush>::stroke(self, SQRT_2 * 0.5 * scale);
//             <Self as Brush>::rotate_right(self, 0.125);
//         }
//     }

//     pub trait MetaBrush: Brush + DerefMut<Target = Self::Inner> + From<Self::Inner> {
//         type Inner: Brush;

//         fn take(self) -> Self::Inner;
//     }

//     // impl<AllMetaBrush: DerefMut + MetaBrush> Brush for AllMetaBrush {
//     //     fn move_to(&mut self, x: f64, y: f64) {
//     //         self.deref_mut().move_to(x, y)
//     //     }

//     //     fn rotate(&mut self, turns_clockwise: f64) {
//     //         self.deref_mut().rotate(turns_clockwise)
//     //     }

//     //     fn stroke(&mut self, scale: f64) {
//     //         self.deref_mut().stroke(scale)
//     //     }

//     //     fn rotate_right(&mut self, turns: f64) {
//     //         self.deref_mut().rotate_right(turns)
//     //     }

//     //     fn rotate_left(&mut self, turns: f64) {
//     //         self.deref_mut().rotate_left(turns)
//     //     }

//     //     fn left_arm(&mut self, scale: f64) {
//     //         self.deref_mut().left_arm(scale)
//     //     }

//     //     fn left_curve(&mut self, scale: f64) {
//     //         self.deref_mut().left_curve(scale)
//     //     }

//     //     fn right_arm(&mut self, scale: f64) {
//     //         self.deref_mut().right_arm(scale)
//     //     }

//     //     fn right_curve(&mut self, scale: f64) {
//     //         self.deref_mut().right_curve(scale)
//     //     }
//     // }

//     #[derive(Default, Debug, Clone)]
//     pub struct SVGPath {
//         path: String,
//         turns: f64,
//         x: f64,
//         y: f64,
//     }

//     impl SVGPath {
//         pub fn new() -> Self {
//             Self::default()
//         }

//         pub fn path(&self) -> &str {
//             &self.path
//         }
//     }

//     impl Brush for SVGPath {
//         fn move_to(&mut self, x: f64, y: f64) {
//             self.path.push_str(&format!("M {}, {}\n", x, y));
//             self.x = x;
//             self.y = y;
//             self.turns = 0.25;
//         }

//         fn rotate(&mut self, turns_clockwise: f64) {
//             self.turns = (((self.turns + turns_clockwise) % 1.0) + 1.0) % 1.0;
//         }

//         fn stroke(&mut self, scale: f64) {
//             // use trig to calculate the dx and dy based on self.turns
//             // and the current scale.
//             let dx = scale * (self.turns * TAU).cos();
//             let dy = scale * (self.turns * TAU).sin();
//             self.x += dx;
//             self.y += dy;
//             self.path.push_str(&format!("L {}, {}\n", self.x, self.y));
//         }
//     }
// }
use templates::*;
mod templates {}
