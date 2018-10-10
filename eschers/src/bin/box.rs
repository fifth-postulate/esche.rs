extern crate eschers;
extern crate svg;

use eschers::canvas::Box as Bx;
use eschers::canvas::*;
use eschers::rendering::canvas::to_svg as box_to_svg;
use eschers::vector::Vector;

fn main() {
    let bx = Bx::new(
        Vector::new(75f64, 75f64),
        Vector::new(250f64, 0f64),
        Vector::new(0f64, 250f64),
    );
    let (_, transformed) = split_box_vertically(0.3f64, &bx);
    let document = box_to_svg((400f64, 400f64), &transformed);
    svg::save("output.svg", &document).unwrap();
}
