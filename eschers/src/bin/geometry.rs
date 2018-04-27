extern crate svg;
extern crate eschers;

use eschers::vector::Vector;
use eschers::canvas::Box as Bx;
use eschers::shape::Shape;
use eschers::fitting::create_picture;
use eschers::rendering::to_svg;

fn main() {
    let bx = Bx::new(
        Vector::new(75f64, 75f64),
        Vector::new(250f64, 0f64),
        Vector::new(0f64, 250f64)
    );
    let shapes = vec!(Shape::Line(Vector::new(0f64, 0f64), Vector::new(1f64, 1f64)));
    let p = create_picture(shapes);
    let picture = p;
    let document = to_svg((400f64, 400f64), &picture(&bx));
    svg::save("output.svg", &document).unwrap();
}
