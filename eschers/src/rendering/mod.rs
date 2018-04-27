//! Turning a `Rendering` into an SVG.

use svg::Document;
use picture::Rendering;

type Bounds = (f64, f64);

/// Create an SVG document from a `Rendering`
pub fn to_svg(bounds: Bounds, _rendering: &Rendering) -> Document {
    Document::new()
        .set("viewbox", (0.0, 0.0, bounds.0, bounds.1))
}
