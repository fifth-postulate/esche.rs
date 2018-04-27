//! Turning a `Rendering` into an SVG.

use svg::Document;
use svg::node::Node;
use svg::node::element as Svg;

use shape::Shape;
use picture::Rendering;

type Bounds = (f64, f64);

/// Create an SVG document from a `Rendering`
pub fn to_svg(bounds: Bounds, rendering: &Rendering) -> Document {
    let mut document = Document::new()
        .set("viewbox", (0.0, 0.0, bounds.0, bounds.1));

    for (shape, style) in rendering {
        let mut node = to_svg_node(&shape);
        node.assign("stroke", "black");
        node.assign("stroke-width", style.stroke_width);
        node.assign("fill", "none");
        document.append(node);
    }

    document
}

fn to_svg_node(shape: &Shape) -> impl Node {
    match shape {
        Shape::Line(line_start, line_end) => {
            Svg::Line::new()
                .set("x1", line_start.x)
                .set("y1", line_start.y)
                .set("x2", line_end.x)
                .set("y2", line_end.y)
        },
        _ => panic!(),
    }
}
