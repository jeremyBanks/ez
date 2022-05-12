use {
    crate::*,
    itertools::Itertools,
    std::{
        cell::RefCell,
        fmt::Display,
        rc::{Rc, Weak},
    },
};

#[derive(Default, Debug, Clone)]
pub struct SVGDocument {
    paths: Vec<SVGPath>,
}

impl SVGDocument {
    pub fn new(paths: Vec<SVGPath>) -> Self {
        Self { paths }
    }
}

impl Display for SVGDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            include_str!("./svg.svg"),
            style = include_str!("./svg.css"),
            script = include_str!("./svg.js"),
            static = self.paths.iter().map(|p| format!("<path d=\"{d}\" />", d = p.path)).join(""),
        )
    }
}

#[derive(Debug, Clone)]
pub struct SVGPath {
    /// total width/height of the SVG context in pixels
    scale: f64,
    /// current X position, as a f64 of .scale, starting from the left
    x: f64,
    /// current Y position, as a f64 of .scale, starting from the top
    y: f64,
    /// current orientation, as a fractional number of revolutions,
    /// clockwise starting at right/+x/east.
    orientation: f64,
    /// the path string so far
    path: String,
}

impl SVGPath {
    pub fn new(x: f64, y: f64, orientation: f64) -> Self {
        let scale = 1024.0;
        let x_px = x * scale;
        let y_px = y * scale;
        Self { scale, x, y, orientation, path: format!("M{x_px},{y_px} ") }
    }
}

impl Default for SVGPath {
    fn default() -> Self {
        Self::new(0.25, 0.25, 0.0)
    }
}

impl SVGDocument {}

// impl crate::brush::Brush for SVGPath {
//     fn rotate(&mut self, revolutions: f64) -> &mut Self {
//         self.orientation += revolutions;
//         self
//     }

//     fn stroke(&mut self, scale: f64) -> &mut Self {
//         let dx = scale * (self.orientation * TAU).cos();
//         let dy = scale * (self.orientation * TAU).sin();
//         self.x += dx;
//         self.y += dy;
//         let dx_px = dx * self.scale;
//         let dy_px = dy * self.scale;
//         match (dx == 0.0, dy == 0.0) {
//             (true, true) => {}
//             (true, false) => self.path += &format!("v{dy_px} "),
//             (false, true) => self.path += &format!("h{dx_px} "),
//             (false, false) => self.path += &format!("l{dx_px},{dy_px} "),
//         };

//         self
//     }
// }
