use {
    crate::*,
    inherent::inherent,
    std::{
        cell::RefCell,
        rc::{Rc, Weak},
    },
};

#[derive(Default, Debug, Clone)]
pub struct SVGDocument {
    paths: Vec<SVGPath>,
}

impl SVGDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_path(&mut self, x: Ratio, y: Ratio, orientation: Revolutions) -> &mut SVGPath {
        self.paths.push(SVGPath::start(x, y, orientation));
        self.paths.last_mut().unwrap()
    }
}

impl ToString for SVGDocument {
    fn to_string(&self) -> String {
        format!(
            include_str!("./svg.svg"),
            style = include_str!("./svg.css"),
            script = include_str!("./svg.js"),
            paths = self.paths.iter().map(|d| format!("<path d=\"\n{d}\n\" />")).join(""),
        )
    }
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
    fn scale(&self) -> Pixels {
        self.scale
    }

    fn path(&self) -> &str {
        &self.path
    }
}

impl crate::brush::Brush for SVGPath {
    fn rotate(&mut self, revolutions: Revolutions) {
        self.orientation += revolutions;
    }

    fn stroke(&mut self, scale: Ratio) {
        let (dx, dy): (Ratio, Ratio) = self.dx_dy(scale, revolutions(0.0));
        self.x += dx;
        self.y += dy;
        let dx_px = (dx * self.scale).get::<pixels>();
        let dy_px = (dy * self.scale).get::<pixels>();
        self.path += &format!("l {dx_px}, {dy_px}\n");
    }
}
