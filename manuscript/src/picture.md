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
instructions and where to place them. If you are really curious you can take a look
at the [documentation](http://localhost:3000/doc/eschers/shape/enum.Shape.html).

The basic shapes are used to provide more abstract drawing instructions such as
letters or grids.

[`Style`](http://localhost:3000/doc/eschers/style/struct.Style.html) instructs
an artist with which style to draw the shapes.

We will treat `Shape` and `Style`, and their derivatives, as black boxes. We
will use them as is, without further inspection.

## Rendering
Single stroke art is a thing, but in general art is made with more elements. A
`Rendering` is just that, a sequences of shapes drawn with a certain style.

`Rendering` is a 
[_type alias_](https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#type-aliases-create-type-synonyms).
It offers a nice shorthand for a type expression, but is otherwise
interchangeable with the long form.

Here is the definition

```rust
pub type Rendering = Vec<(Shape, Style)>;
```

## Picture
Let's take a look at an other alias, this time defined in the signature of the
`turn` function. For reference we repeat the definition below.

```rust
where Picture: Fn(&Bx) -> Rendering
```

A picture is a function that borrows a box, the one defined a few chapters back,
and returns a `Rendering`, i.e. a sequence of shapes to draw.

This is the most important abstraction that we will introduce. All following
types are in place to make these abstraction usable and safe.

## Rc
It you take a look at the type signature of `turn` you will notice that the
`Picture` is wrapped in a `Rc`. It is a

> A single-threaded reference-counting pointer. 

As can be seen from the [`Rc` documentation](https://doc.rust-lang.org/std/rc/struct.Rc.html). 

The reason we need a reference counting pointer to the `Picture` here is
two-fold.

### Picture can not be owned 
The first reason we need an reference counting pointer to the `Picture` is that
we might want to reuse the picture. If you look back at Eschers square limit,
you could see a lot of repetition in the image. As if a stamp is used to create
the collage of fishes.

This means that we can not take ownership of `Picture`, because otherwise other
parts of our program can not reuse it.

### Picture can not be referenced
Usually a possible solution to not owning data is to take a reference.
Unfortunately, that is not an option here. It is a bit involved, but it has to
do with `lifetimes`.

TODO explain why lifetimes are in the way.

### Rc to the rescue
Luckily, with a reference counting pointer, these problems go away. We do trade
something for it. We can't use our program in a multi-threaded environment. We
could if we really wanted, but for this workshop we are not really interested in
multi-threading.

The second is speed. Reference counting happens at run-time. This has a little
overhead that we need to pay each time we run the program. This isn't a concern
for us as well.

## Blackbox
We have looked into the reason why `Picture` is defined as it is. With that
knowledge under our belt, we continue with our goal. Recreating Eschers square
limit.
