//! Creating a `Picture` from `Shape`s

use canvas::Box as Bx;
use shape::Shape;
use style::Style;
use picture::Rendering;

/// Creates a `Picture`, i.e. a `Fn(canvas::Box) -> Vec<(Shape, Style)>`, from
/// `Vec<Shape>`.
pub fn create_picture(_shapes: Vec<Shape>) -> impl Fn(&Bx) -> Rendering {
    |bx: &Bx| {
        let _style = style_for(&bx);
        Vec::new()
    }
}

fn style_for(bx: &Bx) -> Style {
    Style::new(stroke_width_for(&bx))
}

fn stroke_width_for(bx: &Bx) -> f64 {
    bx.b.length().max(bx.c.length()) / 80f64
}
