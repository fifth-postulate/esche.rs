//! An assortment of letters.

use vector::Vector;
use super::Shape;

pub fn d() -> Vec<Shape> {
    let mut shapes = vec!();
    let inner = inner_d_shape();
    let outer = outer_d_shape();

    shapes.push(inner);
    shapes.push(outer);

    shapes
}

fn outer_d_shape() -> Shape {
    let points = vec![
        Vector::new(0.3, 0.2),
        Vector::new(0.3, 0.5),
        Vector::new(0.4, 0.6),
        Vector::new(0.6, 0.6),
        Vector::new(0.6, 0.9),
        Vector::new(0.7, 0.9),
        Vector::new(0.7, 0.1),
        Vector::new(0.4, 0.1)
    ];
    Shape::Polygon(points)
}

fn inner_d_shape() -> Shape {
    let points = vec![
        Vector::new(0.40, 0.24),
        Vector::new(0.40, 0.46),
        Vector::new(0.44, 0.50),
        Vector::new(0.60, 0.50),
        Vector::new(0.60, 0.20),
        Vector::new(0.44, 0.20)
    ];
    Shape::Polygon(points)
}
