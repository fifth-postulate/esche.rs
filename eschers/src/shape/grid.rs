//! An assortment of grids

use super::Shape;
use vector::Vector;

/// A rectangular grid with `m` horizontal dividing lines and `n` vertical dividing lines
pub fn rectangular(m: u8, n: u8) -> Vec<Shape> {
    let mut result = vec![];
    for index in 0..(n + 1) {
        let x = (index as f64) / (n as f64);
        result.push(Shape::Line(Vector::new(x, 0.0), Vector::new(x, 1.0)))
    }
    for index in 0..(m + 1) {
        let y = (index as f64) / (m as f64);
        result.push(Shape::Line(Vector::new(0.0, y), Vector::new(1.0, y)))
    }
    result
}

/// A square grid  with `m` dividing lines both horizontally an vertically
pub fn square(m: u8) -> Vec<Shape> {
    rectangular(m, m)
}
