//! Primitives that can be drawn

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
