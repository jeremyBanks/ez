#![allow(non_camel_case_types)]

use uom::si;

pub type Revolutions = si::f64::Angle;
pub type revolutions = si::angle::revolution;
pub fn revolutions(value: f64) -> Revolutions {
    Revolutions::new::<revolutions>(value)
}

pub type Ratio = si::f64::Ratio;
pub type ratio = si::ratio::ratio;
pub fn ratio(value: f64) -> Ratio {
    Ratio::new::<ratio>(value)
}

pub type Pixels = si::f64::Length;
pub type pixels = si::length::point_computer;
pub fn pixels(value: impl Into<f64>) -> Pixels {
    Pixels::new::<pixels>(value.into())
}

trait Surface {
    type Output;
    fn start(x: Ratio, y: Ratio, orientation: Revolutions) -> Self;
    fn stroke(&mut self, distance: Ratio);
    fn rotate(&mut self, revolutions: Revolutions);
    fn end(self) -> Self::Output;
}

#[derive(Default, Debug, Clone)]
pub struct SVGDocument {
    paths: Vec<SVGPath>,
}

impl SVGDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, x: Ratio, y: Ratio, orientation: Revolutions) -> &mut SVGPath {
        self.paths.push(SVGPath::start(x, y, orientation));
        self.paths.last_mut().unwrap()
    }
}

impl ToString for SVGDocument {
    fn to_string(&self) -> String {
        format!("{self:#?}")
    }
}

#[derive(Debug, Clone)]
pub struct SVGPath {
    path: String,
    x: Ratio,
    y: Ratio,
    orientation: Revolutions,
}

impl SVGPath {
    fn scale() -> Pixels {
        pixels(1024)
    }

    pub fn dx_dy(&self, scale: Ratio, revolutions: Revolutions) -> (Ratio, Ratio) {
        let orientation: Revolutions = self.orientation + revolutions;
        let dx: Ratio = scale * orientation.cos();
        let dy: Ratio = scale * orientation.sin();
        (dx, dy)
    }

    fn x_px(&self) -> f64 {
        (self.x * Self::scale()).get::<pixels>()
    }

    fn y_px(&self) -> f64 {
        (self.y * Self::scale()).get::<pixels>()
    }
}

impl Surface for SVGPath {
    type Output = String;

    fn start(x: Ratio, y: Ratio, orientation: Revolutions) -> SVGPath {
        let mut svg_path = SVGPath { path: String::new(), x, y, orientation };

        let x_px = svg_path.x_px();
        let y_px = svg_path.y_px();
        svg_path.path += &format!("M {x_px}, {y_px}\n");
        svg_path
    }

    fn rotate(&mut self, revolutions: Revolutions) {
        self.orientation += revolutions;
    }

    fn stroke(&mut self, scale: Ratio) {
        let (dx, dy): (Ratio, Ratio) = self.dx_dy(scale, revolutions(0.0));
        self.x += dx;
        self.y += dy;
        let x_px = self.x_px();
        let y_px = self.y_px();
        self.path += &format!("L {x_px}, {y_px}\n");
    }

    fn end(self) -> String {
        self.path
    }
}
