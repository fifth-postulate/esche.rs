//! Descriptions of scenes

use std::rc::Rc;

use canvas::Box as Bx;
use canvas::*;
use shape::Shape;
use style::Style;

/// A collection of Shapes to draw
pub type Rendering = Vec<(Shape, Style)>;

/// the blank picture
pub fn blank() -> Rc<impl Fn(&Bx) -> Rendering> {
    Rc::new(|_bx: &Bx| {
        Vec::new()
    })
}

/// Turn the picture
pub fn turn<Picture>(picture: Rc<Picture>) -> Rc<impl Fn(&Bx) -> Rendering>
where Picture: Fn(&Bx) -> Rendering{
    let p = picture.clone();
    Rc::new(move |bx: &Bx| {
        let turned_box = turn_box(&bx);
        p(&turned_box)
    })
}

/// Flip the picture
pub fn flip<Picture>(picture: Rc<Picture>) -> Rc<impl Fn(&Bx) -> Rendering>
where Picture: Fn(&Bx) -> Rendering{
    let p = picture.clone();
    Rc::new(move |bx: &Bx| {
        let flipped_box = flip_box(&bx);
        p(&flipped_box)
    })
}

/// Toss the picture
pub fn toss<Picture>(picture: Rc<Picture>) -> Rc<impl Fn(&Bx) -> Rendering>
where Picture: Fn(&Bx) -> Rendering{
    let p = picture.clone();
    Rc::new(move |bx: &Bx| {
        let tossed_box = toss_box(&bx);
        p(&tossed_box)
    })
}

/// Stack pictures above each other according to weight
pub fn above_ratio<P, Q>(picture_p: Rc<P>, picture_q: Rc<Q>, m: u8, n: u8) -> Rc<impl Fn(&Bx) -> Rendering>
where P: Fn(&Bx) -> Rendering, Q: Fn(&Bx) -> Rendering {
    let p = picture_p.clone();
    let q = picture_q.clone();
    Rc::new(move |bx: &Bx| {
        let factor = m as f64 / ((m + n) as f64);
        let (top, bottom) = split_box_horizontally(factor, &bx);
        let mut result = vec!();
        result.extend(p(&top));
        result.extend(q(&bottom));
        result
    })
}

/// Stack pictures above each other with equal weight
pub fn above<P, Q>(p: Rc<P>, q: Rc<Q>) -> Rc<impl Fn(&Bx) -> Rendering>
where P: Fn(&Bx) -> Rendering, Q: Fn(&Bx) -> Rendering {
    above_ratio(p, q, 1, 1)
}

/// Stack pictures beside each other according to weight
pub fn beside_ratio<P, Q>(picture_p: Rc<P>, picture_q: Rc<Q>, m: u8, n: u8) -> Rc<impl Fn(&Bx) -> Rendering>
where P: Fn(&Bx) -> Rendering, Q: Fn(&Bx) -> Rendering {
    let p = picture_p.clone();
    let q = picture_q.clone();
    Rc::new(move |bx: &Bx| {
        let factor = m as f64 / ((m + n) as f64);
        let (left, right) = split_box_vertically(factor, &bx);
        let mut result = vec!();
        result.extend(p(&left));
        result.extend(q(&right));
        result
    })
}

/// Stack pictures above each other with equal weight
pub fn beside<P, Q>(p: Rc<P>, q: Rc<Q>) -> Rc<impl Fn(&Bx) -> Rendering>
where P: Fn(&Bx) -> Rendering, Q: Fn(&Bx) -> Rendering {
    beside_ratio(p, q, 1, 1)
}

