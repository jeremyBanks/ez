use uom::si;

pub type Revolutions = si::f64::Angle;
#[allow(non_camel_case_types)]
pub type revolutions = si::angle::revolution;
pub fn revolutions(value: f64) -> Revolutions {
    Revolutions::new::<revolutions>(value)
}

pub type Ratio = si::f64::Ratio;
#[allow(non_camel_case_types)]
pub type ratio = si::ratio::ratio;
pub fn ratio(value: f64) -> Ratio {
    Ratio::new::<ratio>(value)
}

pub type Pixels = si::f64::Length;
#[allow(non_camel_case_types)]
pub type pixels = si::length::point_computer;
pub fn pixels(value: impl Into<f64>) -> Pixels {
    Pixels::new::<pixels>(value.into())
}
