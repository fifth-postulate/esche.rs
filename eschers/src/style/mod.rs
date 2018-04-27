//! Styling of Pictures

/// The various styling options
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// Width of a pen stroke
    pub stroke_width: f64
}

impl Style {
    /// Create a `Style` with a given stroke width
    pub fn new(stroke_width: f64) -> Style {
        Style { stroke_width }
    }
}
