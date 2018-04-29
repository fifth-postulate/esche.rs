//! Descriptions of scenes

use canvas::Box as Bx;
use canvas::*;
use shape::Shape;
use style::Style;

/// A collection of Shapes to draw
pub type Rendering = Vec<(Shape, Style)>;

/// the blank picture
pub fn blank() -> impl Fn(&Bx) -> Rendering {
    |_bx: &Bx| {
        Vec::new()
    }
}

/// Turn the picture
pub fn turn<Picture>(p: Picture) -> impl Fn(&Bx) -> Rendering
where Picture: Fn(&Bx) -> Rendering{
    move |bx: &Bx| {
        let turned_box = turn_box(&bx);
        p(&turned_box)
    }
}

/// Flip the picture
pub fn flip<Picture>(p: Picture) -> impl Fn(&Bx) -> Rendering
where Picture: Fn(&Bx) -> Rendering{
    move |bx: &Bx| {
        let flipped_box = flip_box(&bx);
        p(&flipped_box)
    }
}
