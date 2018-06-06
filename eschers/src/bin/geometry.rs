extern crate svg;
extern crate eschers;

use eschers::vector::Vector;
use eschers::canvas::Box as Bx;
use eschers::shape::{Shape, letter, escher, grid};
use eschers::picture::*;
use eschers::fitting::create_picture;
use eschers::rendering::to_svg;

fn main() {
    let bx = Bx::new(
        Vector::new(0f64, 0f64),
        Vector::new(400f64, 0f64),
        Vector::new(0f64, 400f64)
    );
    let shapes = letter::d();
    let source = create_picture(shapes);
    let picture = turn(source);
    let document = to_svg((400f64, 400f64), &picture(&bx));
    svg::save("output.svg", &document).unwrap();
}
