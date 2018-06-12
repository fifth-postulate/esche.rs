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

## Rendering
Single stroke art is a thing, but in general art is made with more elements. A
`Rendering` is just that a sequences of shapes drawn with a certain style.

`Rendering` is a 
[_type alias_](https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#type-aliases-create-type-synonyms).
It offers a nice shorthand for a type expression, but is otherwise
interchangeable with the long form.

## Picture
Let's take a look at an other alias, this time defined in the signature of the
`turn` function. For reference we repeat the definition below.

```rust
where Picture: Fn(&Bx) -> Rendering
```

A picture is a function that borrows a box, the one defined a few chapters back,
and returns a `Rendering`, i.e. a sequence of shapes to draw.

This is the most important abstraction that we will introduce. All following
types are in place to make these abstraction usable and safe to use.
