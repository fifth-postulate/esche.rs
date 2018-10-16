//! Turning a `Box` into an SVG.

use svg::node::element as Svg;
use svg::node::Node;
use svg::Document;

use canvas::Box as Bx;

type Bounds = (f64, f64);

/// Create an SVG document from a `Rendering`
pub fn to_svg(bounds: Bounds, bx: &Bx) -> Document {
    let mut document = Document::new().set("viewbox", (0.0, 0.0, bounds.0, bounds.1));

    let mut defs = Svg::Definitions::new();
    let mut marker = Svg::Marker::new()
        .set("id", "red-triangle")
        .set("viewBox", "0 0 10 10")
        .set("refX", 10)
        .set("refY", 5)
        .set("markerWidth", 6)
        .set("markerHeight", 6)
        .set("orient", "auto");
    let mut path = Svg::Path::new()
        .set("d", "M 0 0 L 10 5 L 0 10 z")
        .set("fill", "red");
    marker.append(path);
    defs.append(marker);

    marker = Svg::Marker::new()
        .set("id", "orange-triangle")
        .set("viewBox", "0 0 10 10")
        .set("refX", 10)
        .set("refY", 5)
        .set("markerWidth", 6)
        .set("markerHeight", 6)
        .set("orient", "auto");
    path = Svg::Path::new()
        .set("d", "M 0 0 L 10 5 L 0 10 z")
        .set("fill", "orange");
    marker.append(path);
    defs.append(marker);

    marker = Svg::Marker::new()
        .set("id", "purple-triangle")
        .set("viewBox", "0 0 10 10")
        .set("refX", 10)
        .set("refY", 5)
        .set("markerWidth", 6)
        .set("markerHeight", 6)
        .set("orient", "auto");
    path = Svg::Path::new()
        .set("d", "M 0 0 L 10 5 L 0 10 z")
        .set("fill", "purple");
    marker.append(path);
    defs.append(marker);
    document.append(defs);

    let mut group = Svg::Group::new().set(
        "transform",
        format!("scale(1,-1) translate(0,{})", -bounds.1),
    );

    let mut node = Svg::Line::new()
        .set("x1", 0.0)
        .set("y1", 0.0)
        .set("x2", bx.a.x)
        .set("y2", bx.a.y);

    node.assign("stroke", "red");
    node.assign("stroke-width", "3");
    node.assign("fill", "red");
    node.assign("marker-end", "url(#red-triangle)");
    group.append(node);

    node = Svg::Line::new()
        .set("x1", bx.a.x)
        .set("y1", bx.a.y)
        .set("x2", bx.a.x + bx.b.x)
        .set("y2", bx.a.y + bx.b.y);

    node.assign("stroke", "orange");
    node.assign("stroke-width", "3");
    node.assign("fill", "none");
    node.assign("marker-end", "url(#orange-triangle)");
    group.append(node);

    node = Svg::Line::new()
        .set("x1", bx.a.x)
        .set("y1", bx.a.y)
        .set("x2", bx.a.x + bx.c.x)
        .set("y2", bx.a.y + bx.c.y);

    node.assign("stroke", "purple");
    node.assign("stroke-width", "3");
    node.assign("fill", "none");
    node.assign("marker-end", "url(#purple-triangle)");
    group.append(node);

    node = Svg::Line::new()
        .set("x1", bx.a.x + bx.b.x)
        .set("y1", bx.a.y + bx.b.y)
        .set("x2", bx.a.x + bx.b.x + bx.c.x)
        .set("y2", bx.a.y + bx.b.y + bx.c.y);

    node.assign("stroke", "gray");
    node.assign("stroke-width", "3");
    node.assign("stroke-dasharray", "3, 5");
    node.assign("fill", "none");
    group.append(node);

    node = Svg::Line::new()
        .set("x1", bx.a.x + bx.c.x)
        .set("y1", bx.a.y + bx.c.y)
        .set("x2", bx.a.x + bx.b.x + bx.c.x)
        .set("y2", bx.a.y + bx.b.y + bx.c.y);

    node.assign("stroke", "gray");
    node.assign("stroke-width", "3");
    node.assign("stroke-dasharray", "3, 5");
    node.assign("fill", "none");
    group.append(node);

    document.append(group);
    document
}
