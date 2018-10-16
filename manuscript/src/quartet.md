# quartet
<img src="image/quartet_of_d.svg" alt="A quartet of transformed ds" width="400px" height="400px">

A quartet of images, our next goal we will take on. As promised, we will not
need to create a box this time. Instead, we our going to use the primitives we
already created ourselves.

Take some time and study the picture. How are the different parts related? Do
any picture primitives spring to mind?

## Implementation
Open `src/picture/mod.rs`. In it you will find the `quartet` function. Below is
it's signature repeated.

```rust
pub fn quartet<P, Q, R, S>(nw: Rc<P>, ne: Rc<Q>, sw: Rc<R>, se: Rc<S>) -> Rc<impl Fn(&Bx) -> Rendering>
where P: Fn(&Bx) -> Rendering, Q: Fn(&Bx) -> Rendering, R: Fn(&Bx) -> Rendering, S: Fn(&Bx) -> Rendering
```

It is hard to look at. The reason for this is that Rust treats every closure as
its own type. So even though the type parameters `P`, `Q`, `R` and `S` seem to
be defined the same, at compile time they could be different.

Either way, the implementation of the quartet should result in a `Rendering` of
the arguments. They are called `nw`, `ne`, `sw` and `se` to help orient them in
the quartet.

When you find the right combination of picture primitives make sure to enjoy
your hard-work and use it in the `geometry` binary.
