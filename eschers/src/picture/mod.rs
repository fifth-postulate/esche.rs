//! Descriptions of scenes

use std::rc::Rc;

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
pub fn turn<Picture>(picture: Rc<Picture>) -> impl Fn(&Bx) -> Rendering
where Picture: Fn(&Bx) -> Rendering{
    let p = picture.clone();
    move |bx: &Bx| {
        let turned_box = turn_box(&bx);
        p(&turned_box)
    }
}

/// Flip the picture
pub fn flip<Picture>(picture: Rc<Picture>) -> impl Fn(&Bx) -> Rendering
where Picture: Fn(&Bx) -> Rendering{
    let p = picture.clone();
    move |bx: &Bx| {
        let flipped_box = flip_box(&bx);
        p(&flipped_box)
    }
}

/// Toss the picture
pub fn toss<'a, Picture>(picture: Rc<Picture>) -> impl Fn(&Bx) -> Rendering
where Picture: Fn(&Bx) -> Rendering{
    let p = picture.clone();
    move |bx: &Bx| {
        let tossed_box = toss_box(&bx);
        p(&tossed_box)
    }
}

