//! Descriptions of scenes

use canvas::Box as Bx;
use shape::Shape;
use style::Style;

/// the blank picture
pub fn blank() -> impl Fn(&Bx) -> Vec<(Shape, Style)> {
    |_bx: &Bx| {
        Vec::new()
    }
}
