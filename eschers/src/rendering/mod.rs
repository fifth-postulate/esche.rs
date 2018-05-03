//! Turning a `Rendering` into an SVG.

pub mod canvas;

use svg::Document;
use svg::node::Node;
use svg::node::element as Svg;
use itertools::Itertools;

use shape::Shape;
use picture::Rendering;

type Bounds = (f64, f64);

/// Create an SVG document from a `Rendering`
pub fn to_svg(bounds: Bounds, rendering: &Rendering) -> Document {
    let mut document = Document::new()
        .set("viewbox", (0.0, 0.0, bounds.0, bounds.1));

    let mut group = Svg::Group::new()
        .set("transform", format!("scale(1,-1) translate(0,{})", -bounds.1));
    for (shape, style) in rendering {
        match shape {
            Shape::Line(line_start, line_end) => {
                let mut node = Svg::Line::new()
                    .set("x1", line_start.x)
                    .set("y1", line_start.y)
                    .set("x2", line_end.x)
                    .set("y2", line_end.y);

                node.assign("stroke", "black");
                node.assign("stroke-width", style.stroke_width);
                node.assign("fill", "none");
                group.append(node);
            },
            Shape::PolyLine(points) => {
                let points = points.iter()
                    .map(|point| format!("{},{}", point.x, point.y))
                    .join(" ");
                let mut node = Svg::Polyline::new()
                    .set("points", points);

                node.assign("stroke", "black");
                node.assign("stroke-width", style.stroke_width);
                node.assign("fill", "none");
                group.append(node);
            },
            Shape::Polygon(points) => {
                let points = points.iter()
                    .map(|point| format!("{},{}", point.x, point.y))
                    .join(" ");
                let mut node = Svg::Polygon::new()
                    .set("points", points);

                node.assign("stroke", "black");
                node.assign("stroke-width", style.stroke_width);
                node.assign("fill", "none");
                group.append(node);
            },
            Shape::Curve(p1, p2, p3, p4) => {
                let d = format!("M{} {} C{} {}, {} {}, {} {}",
                                p1.x, p1.y,
                                p2.x, p2.y,
                                p3.x, p3.y,
                                p4.x, p4.y,
                );
                let mut node = Svg::Path::new()
                    .set("d", d);

                node.assign("stroke", "black");
                node.assign("stroke-width", style.stroke_width);
                node.assign("fill", "none");
                group.append(node);
            },
            Shape::Path(start, cntrls) => {
                let controls = cntrls.iter()
                    .map(|control| format!("C{} {}, {} {}, {} {}",
                                           control.mid_point1.x, control.mid_point1.y,
                                           control.mid_point2.x, control.mid_point2.y,
                                           control.end_point.x,  control.end_point.y,
                    ))
                    .join(" ");
                let d = format!("M{},{} {}", start.x, start.y, controls);
                let mut node = Svg::Path::new()
                    .set("d", d);

                node.assign("stroke", "black");
                node.assign("stroke-width", style.stroke_width);
                node.assign("fill", "none");
                group.append(node);
            },
        }
    }

    document.append(group);
    document
}
