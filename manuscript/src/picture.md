# Picture
In the last chapter we looked at the interesting signature of the `turn`
function. We will repeated it here.

```rust
pub fn turn<Picture>(picture: Rc<Picture>) -> Rc<impl Fn(&Bx) -> Rendering>
where Picture: Fn(&Bx) -> Rendering 
```

There is a lot going on. There is even a piece of information missing. I.e.
`Rendering` is a type alias.

```rust
pub type Rendering = Vec<(Shape, Style)>;
```

Let's take some time to think and see what this signature is all about.

## Shape & Style
`Shape` are primitive drawing instructions. Various sort of lines and
instructions where to place them. If you are really curious you can take a look
at the [documentation](http://localhost:3000/doc/eschers/shape/enum.Shape.html).

The basic shapes are used to provide more abstract drawing instructions such as
letters or grids.

[`Style`](http://localhost:3000/doc/eschers/style/struct.Style.html) instructs
an artist with which style to draw the shapes.

We will treat `Shape` and `Style`, and their derivatives, as black boxes. We
will use them as is, without further inspection. 
