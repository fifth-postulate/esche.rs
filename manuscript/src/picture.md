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
