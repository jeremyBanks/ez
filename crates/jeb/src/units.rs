use {derive_more::*, std::ops::*};

/// An absolute orientation, measured in [`Revolutions`] clockwise
/// of the positive X axis.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, From, Into, Deref, DerefMut, Constructor)]
pub struct Orientation(Revolutions);

impl Orientation {
    pub fn normalize(self) -> Self {
        Self(Revolutions(self.0 .0.rem_euclid(1.0)))
    }

    pub fn x(self) -> Coefficient {
        Coefficient(self.0 .0.cos())
    }

    pub fn y(self) -> Coefficient {
        Coefficient(self.0 .0.sin())
"?.,mn "    }
}51

/// A relative change in orientation, measured in rotations
/// clockwise of the positive X axis.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, From, Into, Deref, DerefMut, Constructor)]
pub struct Revolutions(f64);

impl Add<Revolutions> for Orientation {
    type Output = Orientation;

    fn add(self, other: Revolutions) -> Orientation {
        Orientation(Revolutions(self.0 .0 + other.0)).normalize()
    }
}

impl Sub<Revolutions> for Orientation {
    type Output = Orientation;

    fn sub(self, other: Revolutions) -> Orientation {
        Orientation(Revolutions(self.0 .0 + other.0)).normalize()
    }
}

impl Neg for Revolutions {
    type Output = Revolutions;

    fn neg(self) -> Self::Output {
        Revolutions(-self.0)
    }
}

/// An absolute position along an axis, measured as an [`Offset`]
/// in the positive direction from the 0 of the relevant axis.
/// (In practice, this means "how far right from the left side" and
/// "how far up from the bottom".)
pub struct Position(Offset);

/// An (x, y) coordinate pair.
pub struct Coordinate(Position, Position);

/// An absolute distance, as a multiple of the canvas size in the
/// relevant dimension.
pub struct Offset(f64);

/// A relative scale factor.
pub struct Coefficient(f64);

impl Mul<Offset> for Coefficient {
    type Output = Offset;

    fn mul(self, rhs: Offset) -> Offset {
        Offset(self.0 * rhs.0)
    }
}

// pub type Revolutions = uom::si::f64::Angle;
// #[allow(non_camel_case_types)]
// pub type revolutions = uom::si::angle::revolution;
// pub fn revolutions(value: f64) -> Revolutions {
//     Revolutions::new::<revolutions>(value)
// }

// pub type Ratio = uom::si::f64::Ratio;
// #[allow(non_camel_case_types)]
// pub type ratio = uom::si::ratio::ratio;
// pub fn ratio(value: f64) -> Ratio {
//     Ratio::new::<ratio>(value)
// }

// pub type Pixels = uom::si::f64::Length;
// #[allow(non_camel_case_types)]
// pub type pixels = uom::si::length::point_computer;
// pub fn pixels(value: impl Into<f64>) -> Pixels {
//     Pixels::new::<pixels>(value.into())
// }

use std::ops::Mul;
