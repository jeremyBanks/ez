//! `Path` trait is used to generate
//!
//!
//!

use {
    crate::units::*,
    inherent::inherent,
    std::{
        cell::RefCell,
        rc::{Rc, Weak},
    },
};

// pub fn main() {
//     let svg = SVGDocument::new();
//     let surface = svg.start(ratio(0.5), ratio(0.5), revolutions(0.125));
//     let brush = surface.brush()
//         .with(RoundedTurns(ratio(0.5)))
//         .with(ZigZagStrokes);

//     let svg = brush.take().take().take().take();
// }

/// Path implementations are used by Brush implementations to generate
/// output (see `SVGPath`)
///
/// Coordinates are all given as `Ratio`s from `0.0` to `1.0`.
trait Path {
    /// The serialized output of the entire path so far.r
    type Output;
    /// Creates a new path, with the given starting point and orientation.
    fn start(x: Ratio, y: Ratio, orientation: Revolutions) -> Self;
    /// Extends the path in the current direction by the given distance.
    fn stroke(&mut self, distance: Ratio);
    /// Rotates the orientation by thee given number of revolutions.
    fn rotate(&mut self, revolutions: Revolutions);
    /// Returns the full serialized output of the path.
    fn get(&self) -> Self::Output;
}


#[derive(Debug, Clone)]
pub struct SVGPath {
    /// total width/height of the SVG context in pixels
    scale: Pixels,
    /// current X position, as a Ratio of .scale, starting from the left
    x: Ratio,
    /// current Y position, as a Ratio of .scale, starting from the top
    y: Ratio,
    /// current orientation, as a fractional number of revolutions,
    /// clockwise starting at right/+x/east.
    orientation: Revolutions,
    /// the path string so far
    path: String,
}

impl SVGPath {
    fn start(x: Ratio, y: Ratio, orientation: Revolutions) -> SVGPath {
        let scale = pixels(1024);

        let x_px = (x * scale).get::<pixels>();
        let y_px = (y * scale).get::<pixels>();

        let path = format!("M {x_px}, {y_px}\n");

        SVGPath {
            scale,
            path,
            x,
            y,
            orientation,
        }
    }

    fn scale(&self) -> Pixels {
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

impl Path for SVGPath {
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

struct PathBrush<Path: self::Path> {
    surface: Rc<Path>,
}

impl<Path: self::Path> PathBrush<Path> {
    pub fn new(surface: Path) -> Self {
        Self { surface }
    }
}

impl<Path: self::Path> Brush for PathBrush<Path> {
    type Output = Path;

    fn stroke(&mut self, distance: Ratio) {
        self.surface.stroke(distance)
    }

    fn rotate(&mut self, revolutions: Revolutions) {
        self.surface.rotate(revolutions)
    }

    fn take(self) -> Path {
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

#[derive(Default, Debug, Clone)]
pub struct SVGDocument {
    paths: Vec<Rc<RefCell<SVGPath>>>,
}

impl SVGDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, x: Ratio, y: Ratio, orientation: Revolutions) -> Rc<RefCell<SVGPath>> {
        self.paths.push(SVGPath::start(x, y, orientation));
        self.paths.last_mut().unwrap()
    }
}

impl ToString for SVGDocument {
    fn to_string(&self) -> String {
        format!("{self:#?}")
    }
}
