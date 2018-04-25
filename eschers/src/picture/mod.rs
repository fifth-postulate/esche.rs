//! Descriptions of scenes

use canvas::Box as Bx;
use shape::Shape;
use style::Style;

type Rendering = Vec<(Shape, Style)>;

/// the blank picture
pub fn blank() -> impl Fn(&Bx) -> Rendering {
    |_bx: &Bx| {
        Vec::new()
    }
}
