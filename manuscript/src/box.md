# Box
We learned about _the grid method_. It allows us to instruct an artist that
knows how copy a picture from a grid, how we want the picture copied. This can
be achieved by controlling the grid the artist will copy the picture into. The
grid method relies on a box. So we better should get to know our boxes.

<img src="image/box.svg" alt="The standard box for reference" width="400px" height="400px">

Above you find a pictorial representation of a _box_. It is defined by three
vectors, `a`, which is red in the picture, `b` which is orange in the picture
and `c` which is purple in the picture. 

```rust
/// A Box represents the area and position that we will draw in.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Box {
    /// Determines the origin of the drawing area, used to position the box.
    pub a: Vector<f64>,
    /// Determines the x axis of the box.
    pub b: Vector<f64>,
    /// Determines the y axis of the box.
    pub c: Vector<f64>,
}
```

The above `struct` is a literal conversion from the picture. If you are
wondering what these `Vector`s are, take a look at the
[documentation](doc/eschers/index.html). 
