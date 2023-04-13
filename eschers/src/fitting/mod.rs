//! Creating a `Picture` from `Shape`s

use std::rc::Rc;

use canvas::Box as Bx;
use picture::Rendering;
use shape::{ControlPoint, Shape};
use style::Style;
use vector::Vector;

/// Creates a `Picture`, i.e. a `Fn(canvas::Box) -> Vec<(Shape, Style)>`, from
/// `Vec<Shape>`.
pub fn create_picture(shapes: Vec<Shape>) -> Rc<impl Fn(&Bx) -> Rendering> {
    Rc::new(move |bx: &Bx| {
        let style = style_for(bx);
        let transformation = transformation_from_box(bx);
        let result: Vec<(Shape, Style)> = shapes
            .iter()
            .map(|shape| map_shape(&transformation, shape))
            .map(|shape| (shape, style))
            .collect();
        result
    })
}

fn style_for(bx: &Bx) -> Style {
    Style::new(stroke_width_for(bx))
}

fn stroke_width_for(bx: &Bx) -> f64 {
    bx.b.length().max(bx.c.length()) / 80f64
}

fn transformation_from_box(cx: &Bx) -> impl Fn(&Vector<f64>) -> Vector<f64> {
    let bx = *cx;
    move |v: &Vector<f64>| bx.a.add(&bx.b.scale(&v.x).add(&bx.c.scale(&v.y)))
}

fn map_shape(transformation: impl Fn(&Vector<f64>) -> Vector<f64>, shape: &Shape) -> Shape {
    match shape {
        Shape::Line(line_start, line_end) => {
            Shape::Line(transformation(line_start), transformation(line_end))
        }
        Shape::PolyLine(points) => {
            let transformed_points: Vec<Vector<f64>> = points.iter().map(transformation).collect();
            Shape::PolyLine(transformed_points)
        }
        Shape::Polygon(points) => {
            let transformed_points: Vec<Vector<f64>> = points.iter().map(transformation).collect();
            Shape::Polygon(transformed_points)
        }
        Shape::Curve(p1, p2, p3, p4) => Shape::Curve(
            transformation(p1),
            transformation(p2),
            transformation(p3),
            transformation(p4),
        ),
        Shape::Path(start, controls) => {
            let new_controls: Vec<ControlPoint> = controls
                .iter()
                .map(|control| {
                    ControlPoint::new(
                        transformation(&control.mid_point1),
                        transformation(&control.mid_point2),
                        transformation(&control.end_point),
                    )
                }).collect();

            Shape::Path(transformation(start), new_controls)
        }
    }
}
