//! Primitives that can be drawn

pub mod escher;
pub mod grid;
pub mod letter;

use vector::Vector;

/// Basic shapes that can be drawn
pub enum Shape {
    Line(Vector<f64>, Vector<f64>),
    PolyLine(Vec<Vector<f64>>),
    Polygon(Vec<Vector<f64>>),
    Curve(Vector<f64>, Vector<f64>, Vector<f64>, Vector<f64>),
    Path(Vector<f64>, Vec<ControlPoint>),
}

/// Control points of a bezier path
pub struct ControlPoint {
    /// First middle control point
    pub mid_point1: Vector<f64>,
    /// Second middle control point
    pub mid_point2: Vector<f64>,
    /// Last control point
    pub end_point: Vector<f64>,
}

impl ControlPoint {
    /// Create a `ControlPoint` from the given `Vector`s.
    pub fn new(
        mid_point1: Vector<f64>,
        mid_point2: Vector<f64>,
        end_point: Vector<f64>,
    ) -> ControlPoint {
        ControlPoint {
            mid_point1,
            mid_point2,
            end_point,
        }
    }
}
