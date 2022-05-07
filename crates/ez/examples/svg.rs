#[ez::ly]
fn main() {
    let mut svg = brush::SVGPath::new();

    // let zig_zag = ZigZagBrush::new(svg);

    svg.move_to(0.5, 0.5);

    jeb(&mut svg, 0.25);

    svg.stroke(0.25);
    svg.rotate_left(0.25);
    svg.stroke(0.25);
    svg.rotate_right(0.50);
    svg.stroke(0.50);

    let doc = templates::document(&format!("<path d=\"\n{}\n\" />", svg.path()));

    println!("{doc}");
}

use patterns::*;
mod patterns {
    use super::brush::Brush;

    pub type Pattern = fn(brush: &mut dyn Brush, scale: f64) -> ();

    pub fn jeb(brush: &mut dyn Brush, scale: f64) {
        fn right(brush: &mut dyn Brush, scale: f64) {
            brush.right_arm(scale);
        }
        fn left(brush: &mut dyn Brush, scale: f64) {
            brush.left_arm(scale);
        }
        fn straight(brush: &mut dyn Brush, scale: f64) {
            brush.stroke(scale);
        }
        let steps = &[
            right, right, left, left, right, right, left, right, left, right, left, right,
            straight, right, right, straight, left, left, straight, left, right, right, left,
            right, left, right, right, left, left, right, right, straight, straight, right, right,
            left, right, left, left, straight, straight, straight, straight, straight, left, left,
            right, left, right, right, straight, straight,
        ];
        for step in steps {
            step(brush, scale);
        }
    }
}

use brush::{Brush, SVGPath};
mod brush {
    use std::{
        f64::consts::{SQRT_2, TAU},
        ops::DerefMut,
    };

    pub trait Brush {
        fn move_to(&mut self, x: f64, y: f64);

        fn rotate(&mut self, turns_clockwise: f64);

        fn stroke(&mut self, scale: f64);

        fn rotate_right(&mut self, turns: f64) {
            <Self as Brush>::rotate(self, turns)
        }

        fn rotate_left(&mut self, turns: f64) {
            <Self as Brush>::rotate(self, -turns)
        }

        fn left_arm(&mut self, scale: f64) {
            <Self as Brush>::stroke(self, 0.500 * scale);
            <Self as Brush>::rotate_left(self, 0.250);
            <Self as Brush>::stroke(self, 0.500 * scale);
        }

        fn left_curve(&mut self, scale: f64) {
            <Self as Brush>::rotate_left(self, 0.125);
            <Self as Brush>::stroke(self, SQRT_2 / 2.000 * scale);
            <Self as Brush>::rotate_left(self, 0.125);
        }

        fn right_arm(&mut self, scale: f64) {
            <Self as Brush>::stroke(self, 0.500 * scale);
            <Self as Brush>::rotate_right(self, 0.250);
            <Self as Brush>::stroke(self, 0.500 * scale);
        }

        fn right_curve(&mut self, scale: f64) {
            <Self as Brush>::rotate_right(self, 0.125);
            <Self as Brush>::stroke(self, SQRT_2 * 0.5 * scale);
            <Self as Brush>::rotate_right(self, 0.125);
        }
    }

    pub trait MetaBrush: Brush + DerefMut<Target = Self::Inner> {
        type Inner: Brush;
    }

    impl<AllMetaBrush: MetaBrush> Brush for AllMetaBrush {
        fn move_to(&mut self, x: f64, y: f64) {
            self.deref_mut().move_to(x, y)
        }

        fn rotate(&mut self, turns_clockwise: f64) {
            self.deref_mut().rotate(turns_clockwise)
        }

        fn stroke(&mut self, scale: f64) {
            self.deref_mut().stroke(scale)
        }

        fn rotate_right(&mut self, turns: f64) {
            self.deref_mut().rotate_right(turns)
        }

        fn rotate_left(&mut self, turns: f64) {
            self.deref_mut().rotate_left(turns)
        }

        fn left_arm(&mut self, scale: f64) {
            self.deref_mut().left_arm(scale)
        }

        fn left_curve(&mut self, scale: f64) {
            self.deref_mut().left_curve(scale)
        }

        fn right_arm(&mut self, scale: f64) {
            self.deref_mut().right_arm(scale)
        }

        fn right_curve(&mut self, scale: f64) {
            self.deref_mut().right_curve(scale)
        }
    }

    #[derive(Default, Debug, Clone)]
    pub struct SVGPath {
        path: String,
        turns: f64,
        x: f64,
        y: f64,
    }

    impl SVGPath {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn path(&self) -> &str {
            &self.path
        }
    }

    impl Brush for SVGPath {
        fn move_to(&mut self, x: f64, y: f64) {
            self.path.push_str(&format!("M {}, {}\n", x, y));
            self.x = x;
            self.y = y;
            self.turns = 0.25;
        }

        fn rotate(&mut self, turns_clockwise: f64) {
            self.turns = (((self.turns + turns_clockwise) % 1.0) + 1.0) % 1.0;
        }

        fn stroke(&mut self, scale: f64) {
            // use trig to calculate the dx and dy based on self.turns
            // and the current scale.
            let dx = scale * (self.turns * TAU).cos();
            let dy = scale * (self.turns * TAU).sin();
            self.x += dx;
            self.y += dy;
            self.path.push_str(&format!("L {}, {}\n", self.x, self.y));
        }
    }
}

use templates::*;
mod templates {
    pub fn document(contents: &str) -> String {
        let style = style();
        let script = script();
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1">
                <style>{style}</style>

                <defs>
                    <pattern id="g1" viewBox="0 0 0.25 0.25" width="0.25" height="0.25">
                        <g class="axis">
                            <line x1="0.0" x2="0.0" y1="0" y2="0.25" stroke-width="0.00125" />
                            <line x1="0.0" x2="0.25" y1="0" y2="0" stroke-width="0.00125" />
                        </g>
                        <g class="diag">
                            <line x1="0.25" x2="0.25" y1="0" y2="0.25" stroke-width="0.00125" />
                            <line x1="0.0" x2="0.25" y1="0.25" y2="0.25" stroke-width="0.00125" />
                            <line x1="0" x2="0.25" y1="0" y2="0.25" stroke-width="0.00125" />
                            <line x1="0.25" x2="0" y1="0" y2="0.25" stroke-width="0.00125" />
                        </g>
                    </pattern>

                    <pattern id="g2" viewBox="0 0 0.50 0.50" width="0.50" height="0.50">
                        <rect fill="url(#g1)" width="0.50" height="0.50" />
                    </pattern>

                    <pattern id="g3" viewBox="0 0 0.50 0.50" width="0.50" height="0.50">
                        <rect fill="url(#g2)" width="0.50" height="0.50" />
                    </pattern>

                    <pattern id="g4" viewBox="0 0 0.50 0.50" width="0.50" height="0.50">
                        <rect fill="url(#g3)" width="0.50" height="0.50" />
                    </pattern>

                    <pattern id="grid" viewBox="0 0 1 1" width="1" height="1">
                        <rect fill="url(#g1)" width="1" height="1" />
                        <rect fill="url(#g2)" width="1" height="1" />
                        <rect fill="url(#g3)" width="1" height="1" />
                        <rect fill="url(#g4)" width="1" height="1" />
                    </pattern>
                </defs>

                <defs class="interactive">
                    <rect x="0" y="0" width="1" height="1" class="background" />
                </defs>

                <g class="static">{contents}</g>

                <defs class="interactive">
                    <rect x="0" y="0" width="1" height="1" class="grid" />
                </defs>

                <script>{script}</script>
            </svg>"#
        )
    }

    fn style() -> String {
        r#"
            svg.interactive {
                background: white;
            }

            .axis line {
                stroke: #CCC;
            }

            .diag line {
                stroke: #EEE;
            }

            path {
                mix-blend-mode: multiply;
                stroke-linecap: round;
                stroke-linejoin: round;
                fill: #F8F8F8;
                stroke-width: 0.002px;
                stroke: black;
            }

            path:not(:only-child):nth-child(6n + 1) {
                stroke: #440;
                fill: #FFFFF0;
            }

            path:not(:only-child):nth-child(6n + 2) {
                stroke: #044;
                fill: #F0FFFF;
            }

            path:not(:only-child):nth-child(6n + 3) {
                    stroke: #404;
                    fill: #FFF0FF;
            }

            path:not(:only-child):nth-child(6n + 4) {
                stroke: #008;
                fill: #F8F8FF;
            }

            path:not(:only-child):nth-child(6n + 5) {
                stroke: #080;
                fill: #F8FFF8;
            }

            path:not(:only-child):nth-child(6n + 6) {
                stroke: #800;
                fill: #FFF8F8;
            }

            .background {
                fill: white;
            }

            .grid {
                mix-blend-mode: multiply;
                fill: url(#grid);
            }
        "#
        .to_owned()
    }

    fn script() -> String {
        r#"
            "use strict";
            document.documentElement.classList.add("interactive");
            for (const def of document.querySelectorAll("defs.interactive")) {
                for (const child of def.children) {
                    def.before(child);
                }
                def.remove();
            }
        "#
        .to_owned()
    }
}
