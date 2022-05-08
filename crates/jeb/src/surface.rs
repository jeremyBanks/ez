#![allow(non_camel_case_types)]

use {inherent::inherent, uom::si};

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

pub fn main() {
    let svg = SVGDocument::new();
    let path = svg.start(ratio(0.5), ratio(0.5), revolutions(0.125));
    let brush = SurfaceBrush::new(path);
    let brush = svg.with(RoundedTurns(ratio(0.5)));
}

trait Surface {
    type Output;
    fn start(x: Ratio, y: Ratio, orientation: Revolutions) -> Self;
    fn stroke(&mut self, distance: Ratio);
    fn rotate(&mut self, revolutions: Revolutions);
    fn take(self) -> Self::Output;
}

trait Brush: Sized {
    type Output;
    fn stroke(&mut self, distance: Ratio);
    fn rotate(&mut self, revolutions: Revolutions);
    fn take(self) -> Self::Output;

    fn with<Behavior: MetaBrushBehavior + Sized>(
        self,
        behaviour: Behavior,
    ) -> MetaBrush<Self, Behavior> {
        MetaBrush::new(behaviour, self)
    }
}

struct SurfaceBrush<Surface: self::Surface> {
    surface: Surface,
}

impl<Surface: self::Surface> SurfaceBrush<Surface> {
    pub fn new(surface: Surface) -> Self {
        Self { surface }
    }
}

impl<Surface: self::Surface> Brush for SurfaceBrush<Surface> {
    type Output = Surface;

    fn stroke(&mut self, distance: Ratio) {
        self.surface.stroke(distance)
    }

    fn rotate(&mut self, revolutions: Revolutions) {
        self.surface.rotate(revolutions)
    }

    fn take(self) -> Surface {
        self.surface
    }
}

struct MetaBrush<Brush: self::Brush, Behavior: MetaBrushBehavior = NoBehavior> {
    brush: Brush,
    behavior: Behavior,
}

impl<Brush: self::Brush, Behavior: MetaBrushBehavior> MetaBrush<Brush, Behavior> {
    fn new(behavior: Behavior, brush: Brush) -> Self {
        Self { brush, behavior }
    }
}

trait MetaBrushBehavior {
    fn stroke(&mut self, inner: &mut impl Brush, distance: Ratio) {
        inner.stroke(distance)
    }

    fn rotate(&mut self, inner: &mut impl Brush, revolutions: Revolutions) {
        inner.rotate(revolutions)
    }
}

impl<Brush: self::Brush, Behavior: MetaBrushBehavior> self::Brush for MetaBrush<Brush, Behavior> {
    type Output = Brush;

    fn stroke(&mut self, distance: Ratio) {
        self.behavior.stroke(&mut self.brush, distance)
    }

    fn rotate(&mut self, revolutions: Revolutions) {
        self.behavior.rotate(&mut self.brush, revolutions)
    }

    fn take(self) -> Self::Output {
        self.brush
    }
}

struct NoBehavior;
impl MetaBrushBehavior for NoBehavior {}

struct TurnsFromArms;
impl MetaBrushBehavior for TurnsFromArms {}

struct TurnsFromCurves;
impl MetaBrushBehavior for TurnsFromCurves {}

#[derive(Clone, Copy, Debug)]
struct RoundedTurns(Ratio);

impl MetaBrushBehavior for RoundedTurns {}

struct ArmsFromStrokes;
impl MetaBrushBehavior for ArmsFromStrokes {}

struct ZigZagStrokes;
impl MetaBrushBehavior for ZigZagStrokes {}

struct CurveyStrokes;
impl MetaBrushBehavior for CurveyStrokes {}

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
        let dx_px = (dx * Self::scale()).get::<pixels>();
        let dy_px = (dy * Self::scale()).get::<pixels>();
        self.path += &format!("l {dx_px}, {dy_px}\n");
    }

    fn take(self) -> String {
        self.path
    }
}
