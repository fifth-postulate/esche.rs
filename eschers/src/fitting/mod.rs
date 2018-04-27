//! Creating a `Picture` from `Shape`s

use vector::Vector;
use canvas::Box as Bx;
use shape::Shape;
use style::Style;
use picture::Rendering;

/// Creates a `Picture`, i.e. a `Fn(canvas::Box) -> Vec<(Shape, Style)>`, from
/// `Vec<Shape>`.
pub fn create_picture(shapes: Vec<Shape>) -> impl Fn(&Bx) -> Rendering {
    move |bx: &Bx| {
        let _style = style_for(&bx);
        let transformation = transformation_from_box(&bx);
        let _result: Vec<Shape> = shapes.iter()
        	.map(|shape| map_shape(&transformation, &shape))
        	.collect();
        Vec::new()
    }
}

fn style_for(bx: &Bx) -> Style {
    Style::new(stroke_width_for(&bx))
}

fn stroke_width_for(bx: &Bx) -> f64 {
    bx.b.length().max(bx.c.length()) / 80f64
}

fn transformation_from_box(cx: &Bx) -> impl Fn(&Vector<f64>) -> Vector<f64> {
	let bx = cx.clone();
	move |v: &Vector<f64>| {
		bx.a.add(&bx.b.scale(&v.x).add(&bx.c.scale(&v.y)))
	}
}

fn map_shape(transformation: impl Fn(&Vector<f64>) -> Vector<f64>, shape: &Shape) -> Shape {
	match shape {
		Shape::Line(line_start, line_end) => Shape::Line(transformation(line_start), transformation(line_end)),
		_ => panic!()
	}
}