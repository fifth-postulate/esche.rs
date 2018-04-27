//! 2D vectors and associated functions
//!
//! Vectors are used to represent locations in the two dimensional plane.
//!
//! Vectors can be added.
//!
//! ```
//! # use eschers::vector::Vector;
//! let u = Vector::new(1f64, 0f64);
//! let v = Vector::new(0f64, 1f64);
//!
//! let w = u.add(&v);
//!
//! assert_eq!(w, Vector::new(1f64, 1f64));
//! ```
//!
//! You can negate a vector.
//!
//! ```
//! # use eschers::vector::Vector;
//! # let u = Vector::new(1f64, 0f64);
//!
//! let w = u.neg();
//!
//! assert_eq!(w, Vector::new(-1f64, 0f64));
//! ```
//!
//! Subtraction of two vectors is also an option.
//!
//! ```
//! # use eschers::vector::Vector;
//! # let u = Vector::new(1f64, 0f64);
//! # let v = Vector::new(0f64, 1f64);
//!
//! let w = u.sub(&v);
//!
//! assert_eq!(w, Vector::new(1f64, -1f64));
//! ```
//!
//! Just as scaling a vector.
//!
//! ```
//! # use eschers::vector::Vector;
//! # let u = Vector::new(1f64, 0f64);
//!
//! let w = u.scale(&2f64);
//!
//! assert_eq!(w, Vector::new(2f64, 0f64));
//! ```
//!
//! Finally with a `Vector<f64>` you can determine their length.
//!
//! ```
//! # use eschers::vector::Vector;
//! let t = Vector::new(3f64, 4f64);
//!
//! assert_eq!(5f64, t.length());
//! ```
//!
//! Note that all the methods that return a `Vector`, do so by returning a new
//! structure.


use std::ops::{Add, Neg, Mul};

/// Representation of two dimensional vector
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vector<T> where T: Clone + Copy {
    /// x-coordinate of the vector
    pub x: T,
    /// y-coordinate of the vector
    pub y: T,
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
