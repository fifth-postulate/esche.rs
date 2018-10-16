//! Escher's famous fish.

use super::Shape;
use vector::Vector;

/// The fish
pub fn fish() -> Vec<Shape> {
    vec![
        create_curve(
            create_vector(0.116, 0.702), // C1
            create_vector(0.260, 0.295), //
            create_vector(0.330, 0.258), //
            create_vector(0.815, 0.078),
        ), //
        create_curve(
            create_vector(0.564, 0.032), // C2
            create_vector(0.730, 0.056), //
            create_vector(0.834, 0.042), //
            create_vector(1.000, 0.000),
        ), //
        create_curve(
            create_vector(0.250, 0.250), // C3
            create_vector(0.372, 0.194), //
            create_vector(0.452, 0.132), //
            create_vector(0.564, 0.032),
        ), //
        create_curve(
            create_vector(0.000, 0.000), // C4
            create_vector(0.110, 0.110), //
            create_vector(0.175, 0.175), //
            create_vector(0.250, 0.250),
        ), //
        create_curve(
            create_vector(-0.250, 0.250), // C5
            create_vector(-0.150, 0.150), //
            create_vector(-0.090, 0.090), //
            create_vector(0.000, 0.000),
        ), //
        create_curve(
            create_vector(-0.250, 0.250), // C6
            create_vector(-0.194, 0.372), //
            create_vector(-0.132, 0.452), //
            create_vector(-0.032, 0.564),
        ), //
        create_curve(
            create_vector(-0.032, 0.564), // C7
            create_vector(0.055, 0.355),  //
            create_vector(0.080, 0.330),  //
            create_vector(0.250, 0.250),
        ), //
        create_curve(
            create_vector(-0.032, 0.564), // C8
            create_vector(-0.056, 0.730), //
            create_vector(-0.042, 0.834), //
            create_vector(0.000, 1.000),
        ), //
        create_curve(
            create_vector(0.000, 1.000), // C9
            create_vector(0.104, 0.938), //
            create_vector(0.163, 0.893), //
            create_vector(0.234, 0.798),
        ), //
        create_curve(
            create_vector(0.234, 0.798), // C10
            create_vector(0.368, 0.650),
            create_vector(0.232, 0.540),
            create_vector(0.377, 0.377),
        ),
        create_curve(
            create_vector(0.377, 0.377), // C11
            create_vector(0.400, 0.350),
            create_vector(0.450, 0.300),
            create_vector(0.500, 0.250),
        ),
        create_curve(
            create_vector(0.500, 0.250), // C12
            create_vector(0.589, 0.217),
            create_vector(0.660, 0.208),
            create_vector(0.766, 0.202),
        ),
        create_curve(
            create_vector(0.766, 0.202), // C13
            create_vector(0.837, 0.107), //
            create_vector(0.896, 0.062), //
            create_vector(1.000, 0.000),
        ), //
        create_curve(
            create_vector(0.234, 0.798), // C14
            create_vector(0.340, 0.792), //
            create_vector(0.411, 0.783), //
            create_vector(0.500, 0.750),
        ), //
        create_curve(
            create_vector(0.500, 0.750), // C15
            create_vector(0.500, 0.625), //
            create_vector(0.500, 0.575), //
            create_vector(0.500, 0.500),
        ), //
        create_curve(
            create_vector(0.500, 0.500), // C16 -
            create_vector(0.460, 0.460), //
            create_vector(0.410, 0.410), //
            create_vector(0.377, 0.377),
        ), //
        create_curve(
            create_vector(0.315, 0.710), // C17 -
            create_vector(0.378, 0.732), //
            create_vector(0.426, 0.726), //
            create_vector(0.487, 0.692),
        ), //
        create_curve(
            create_vector(0.340, 0.605), // C18 -
            create_vector(0.400, 0.642), //
            create_vector(0.435, 0.647), //
            create_vector(0.489, 0.626),
        ), //
        create_curve(
            create_vector(0.348, 0.502), // C19 -
            create_vector(0.400, 0.564), //
            create_vector(0.422, 0.568), //
            create_vector(0.489, 0.563),
        ), //
        create_curve(
            create_vector(0.451, 0.418), // C20 -
            create_vector(0.465, 0.400), //
            create_vector(0.480, 0.385), //
            create_vector(0.490, 0.381),
        ), //
        create_curve(
            create_vector(0.421, 0.388), // C21 -
            create_vector(0.440, 0.350), //
            create_vector(0.455, 0.335), //
            create_vector(0.492, 0.325),
        ), //
        create_curve(
            create_vector(-0.170, 0.237), // C22 -
            create_vector(-0.125, 0.355), //
            create_vector(-0.065, 0.405), //
            create_vector(0.002, 0.436),
        ), //
        create_curve(
            create_vector(-0.121, 0.188), // C23 -
            create_vector(-0.060, 0.300), //
            create_vector(-0.030, 0.330), //
            create_vector(0.040, 0.375),
        ), //
        create_curve(
            create_vector(-0.058, 0.125), // C24 -
            create_vector(-0.010, 0.240), //
            create_vector(0.030, 0.280),  //
            create_vector(0.100, 0.321),
        ), //
        create_curve(
            create_vector(-0.022, 0.063), // C25 -
            create_vector(0.060, 0.200),  //
            create_vector(0.100, 0.240),  //
            create_vector(0.160, 0.282),
        ), //
        create_curve(
            create_vector(0.053, 0.658), // C26 -
            create_vector(0.075, 0.677), //
            create_vector(0.085, 0.687), //
            create_vector(0.098, 0.700),
        ), //
        create_curve(
            create_vector(0.053, 0.658), // C27
            create_vector(0.042, 0.710), //
            create_vector(0.042, 0.760), //
            create_vector(0.053, 0.819),
        ), //
        create_curve(
            create_vector(0.053, 0.819), // C28 -
            create_vector(0.085, 0.812), //
            create_vector(0.092, 0.752), //
            create_vector(0.098, 0.700),
        ), //
        create_curve(
            create_vector(0.130, 0.718), // C29 -
            create_vector(0.150, 0.730), //
            create_vector(0.175, 0.745), //
            create_vector(0.187, 0.752),
        ), //
        create_curve(
            create_vector(0.130, 0.718), // C30 -
            create_vector(0.110, 0.795), //
            create_vector(0.110, 0.810), //
            create_vector(0.112, 0.845),
        ), //
        create_curve(
            create_vector(0.112, 0.845), // C31 -
            create_vector(0.150, 0.805), //
            create_vector(0.172, 0.780), //
            create_vector(0.187, 0.752),
        ),
    ]
}

fn create_vector(x: f64, y: f64) -> Vector<f64> {
    Vector::new(x, y)
}

fn create_curve(v1: Vector<f64>, v2: Vector<f64>, v3: Vector<f64>, v4: Vector<f64>) -> Shape {
    Shape::Curve(v1, v2, v3, v4)
}
