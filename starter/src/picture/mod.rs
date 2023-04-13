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
    Rc::new(|_bx: &Bx| Vec::new())
}

/// Turn the picture
pub fn turn<Picture>(p: Rc<Picture>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    Picture: Fn(&Bx) -> Rendering,
{
    Rc::new(move |bx: &Bx| {
        let turned_box = turn_box(bx);
        p(&turned_box)
    })
}

/// Flip the picture
pub fn flip<Picture>(p: Rc<Picture>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    Picture: Fn(&Bx) -> Rendering,
{
    blank()
}

/// Toss the picture
pub fn toss<Picture>(p: Rc<Picture>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    Picture: Fn(&Bx) -> Rendering,
{
    blank()
}

/// Stack pictures above each other according to weight
pub fn above_ratio<P, Q>(
    p: Rc<P>,
    q: Rc<Q>,
    m: u8,
    n: u8,
) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
{
    blank()
}

/// Stack pictures above each other with equal weight
pub fn above<P, Q>(p: Rc<P>, q: Rc<Q>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
{
    blank()
}

/// Stack pictures beside each other according to weight
pub fn beside_ratio<P, Q>(
    p: Rc<P>,
    q: Rc<Q>,
    m: u8,
    n: u8,
) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
{
    blank()
}

/// Stack pictures above each other with equal weight
pub fn beside<P, Q>(p: Rc<P>, q: Rc<Q>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
{
    blank()
}

/// Create a quartet of pictures
pub fn quartet<P, Q, R, S>(
    nw: Rc<P>,
    ne: Rc<Q>,
    sw: Rc<R>,
    se: Rc<S>,
) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
    R: Fn(&Bx) -> Rendering,
    S: Fn(&Bx) -> Rendering,
{
    blank()
}

/// Create a nonet of pictures
pub fn nonet<P, Q, R, S, T, U, V, W, X>(
    nw: Rc<P>,
    nm: Rc<Q>,
    ne: Rc<R>,
    mw: Rc<S>,
    mm: Rc<T>,
    me: Rc<U>,
    sw: Rc<V>,
    sm: Rc<W>,
    se: Rc<X>,
) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
    R: Fn(&Bx) -> Rendering,
    S: Fn(&Bx) -> Rendering,
    T: Fn(&Bx) -> Rendering,
    U: Fn(&Bx) -> Rendering,
    V: Fn(&Bx) -> Rendering,
    W: Fn(&Bx) -> Rendering,
    X: Fn(&Bx) -> Rendering,
{
    blank()
}

fn column<P, Q, R>(n: Rc<P>, m: Rc<Q>, s: Rc<R>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
    R: Fn(&Bx) -> Rendering,
{
    blank()
}

fn row<P, Q, R>(w: Rc<P>, m: Rc<Q>, e: Rc<R>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
    R: Fn(&Bx) -> Rendering,
{
    blank()
}

/// Place two pictures over each other
pub fn over<P, Q>(p: Rc<P>, q: Rc<Q>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
{
    blank()
}

/// The T-tile
pub fn ttile<P>(p: Rc<P>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
{
    blank()
}

/// The T-tile
pub fn utile<P>(p: Rc<P>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
{
    blank()
}

/// The side of the square limit
pub fn side<P>(p: Rc<P>, n: u8) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
{
    blank()
}

/// The corner of the square limit
pub fn corner<P>(p: Rc<P>, n: u8) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
{
    blank()
}

/// The ultimate goal: Escher's Square Limit
pub fn square_limit<P>(p: Rc<P>, n: u8) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
{
    blank()
}
