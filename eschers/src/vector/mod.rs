//! 2D vectors and associated functions

use std::ops::{Add, Neg, Mul};

/// Representation of two dimensional vector
pub struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> where T: Copy + Add<Output=T> + Neg<Output=T> + Mul<Output=T> {
    /// Create a `Vector` with Cartesian coordinates `x` and `y`
    pub fn new(x: T, y: T) -> Self {
        Vector { x, y }
    }

    /// Add `other` component-wise to `self`, returning a new `Vector`
    pub fn add(&self, other: &Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;

        Vector::new(x, y)
    }

    /// Negate `self` component-wise, returning a new `Vector`
    pub fn neg(&self) -> Self {
        let x = -self.x;
        let y = -self.y;

        Vector::new(x, y)
    }

    /// Subtract `other` component-wise from `self`, returning a new `Vector`
    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    /// Scale `self` uniformly by `factor`, returning a new `Vector`
    pub fn scale(&self, factor: &T) -> Self {
        let x = self.x * *factor;
        let y = self.y * *factor;

        Vector::new(x, y)
    }
}

impl Vector<f64> {
    /// Calculate the length of a `Vector<f64>`
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
