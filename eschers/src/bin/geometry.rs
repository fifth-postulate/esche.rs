extern crate eschers;
extern crate svg;

use eschers::canvas::Box as Bx;
use eschers::fitting::create_picture;
use eschers::picture::*;
use eschers::rendering::to_svg;
use eschers::shape::{escher, grid, letter, Shape};
use eschers::vector::Vector;

fn main() {
    let bx = Bx::new(
        Vector::new(75f64, 75f64),
        Vector::new(250f64, 0f64),
        Vector::new(0f64, 250f64),
    );
    let shapes = escher::fish();
    let source = create_picture(shapes);
    let picture = square_limit(source, 3);
    let document = to_svg((400f64, 400f64), &picture(&bx));
    svg::save("output.svg", &document).unwrap();
}
