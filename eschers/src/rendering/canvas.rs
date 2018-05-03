//! Turning a `Box` into an SVG.

use svg::Document;
use svg::node::Node;
use svg::node::element as Svg;

use canvas::Box as Bx;
use picture::Rendering;

type Bounds = (f64, f64);

/// Create an SVG document from a `Rendering`
pub fn to_svg(bounds: Bounds, bx: &Bx) -> Document {
    let mut document = Document::new()
        .set("viewbox", (0.0, 0.0, bounds.0, bounds.1));

    let mut group = Svg::Group::new()
        .set("transform", format!("scale(1,-1) translate(0,{})", -bounds.1));

    let mut node = Svg::Line::new()
        .set("x1", 0.0)
        .set("y1", 0.0)
        .set("x2", bx.a.x)
        .set("y2", bx.a.y);

    node.assign("stroke", "black");
    node.assign("stroke-width", "3");
    node.assign("fill", "none");
    group.append(node);

    node = Svg::Line::new()
        .set("x1", bx.a.x)
        .set("y1", bx.a.y)
        .set("x2", bx.a.x + bx.b.x)
        .set("y2", bx.a.y + bx.b.y);

    node.assign("stroke", "black");
    node.assign("stroke-width", "3");
    node.assign("fill", "none");
    group.append(node);

    node = Svg::Line::new()
        .set("x1", bx.a.x)
        .set("y1", bx.a.y)
        .set("x2", bx.a.x + bx.c.x)
        .set("y2", bx.a.y + bx.c.y);

    node.assign("stroke", "black");
    node.assign("stroke-width", "3");
    node.assign("fill", "none");
    group.append(node);

    document.append(group);
    document
}

