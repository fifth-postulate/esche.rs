//! The canvas that we will draw on

use vector::Vector;

/// A Box represents the area and position that we will draw in.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Box {
    /// Determines the origin of the drawing area, used to position the box.
    pub a: Vector<f64>,
    /// Determines the x axis of the box.
    pub b: Vector<f64>,
    /// Determines the y axis of the box.
    pub c: Vector<f64>,
}

impl Box {
    /// Create a box from corresponding vectors
    pub fn new(a: Vector<f64>, b: Vector<f64>, c: Vector<f64>) -> Box {
        Box { a, b, c }
    }
}

pub fn identity(bx: &Box) -> Box {
    Box::new(bx.a, bx.b, bx.c)
}