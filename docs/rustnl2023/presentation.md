layout: true
class: middle, center

---

# Waiter, there are fish in my Rust

---

# `::<>`

---

![Eschers Square Limit](image/escher-square-limit.jpg)

---

![Rust Logo](image/rust-logo-blk.png)

---

![Rust Logo With a Grid overlaid](image/grid-logo.png)

---

![Rust Logo With a Grid overlaid](image/grid-logo.png)
![A grid](image/grid.png)

---

![Rust Logo With a Grid overlaid](image/grid-logo.png)
![A grid, with a single cell copied](image/grid-logo-copy01.png)

---

![Rust Logo With a Grid overlaid](image/grid-logo.png)
![A grid, with two cells copied](image/grid-logo-copy02.png)

---

![Rust Logo With a Grid overlaid](image/grid-logo.png)
![A grid, with three cells copied](image/grid-logo-copy03.png)

---

![Rust Logo With a Grid overlaid](image/grid-logo.png)
![A copied Rust Logo](image/grid-logo.png)

---

![Rust Logo With a Grid overlaid](image/grid-logo.png)
![Rust Logo non-uniformly scaled](image/grid-logo-non-uniformly-scaled.png)

---

![Rust Logo With a Grid overlaid](image/grid-logo.png)
![Rust Logo Sheared](image/grid-logo-shear.png)

---

![Rust Logo With a Grid overlaid](image/grid-logo.png)
![Rust Logo transformed](image/grid-logo-combination.png)

---

![A box; the way we instruct a painter](image/box.png)

--

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

---

![A letter d; an asymmetric letter](image/d.png)

---

![A letter d turned](image/d_turned.png)

---

![A box; the way we instruct a painter](image/box.png)
![A turned box](image/box_turned.png)

--

![vector addition](image/vector_sum.png)

---

```rust
pub fn turn_box(bx: &Box) -> Box {
    Box::new(
        bx.a.add(&bx.b),
        bx.c,
        bx.b.neg()
    )
}
```

---

![A letter d flipped](image/d_flipped.png)

---

![A box; the way we instruct a painter](image/box.png)
![A flipped box](image/box_flipped.png)

---

```rust
pub fn flip_box(bx: &Box) -> Box {
    Box::new(
        bx.a.add(&bx.b),
        bx.b.neg(),
        bx.c)
    )
}
```

---

```rust
pub fn flip<Picture>(p: Rc<Picture>)
    -> Rc<impl Fn(&Bx) -> Rendering>
where
    Picture: Fn(&Bx) -> Rendering,
{
    Rc::new(move |bx: &Bx| {
        let flipped_box = flip_box(bx);
        p(&flipped_box)
    })
}
```

---

```rust
pub fn flip<Picture>(picture: Rc<Picture>)
    -> Rc<impl Fn(&Bx) -> Rendering>
where Picture: Fn(&Bx) -> Rendering
```

--

```rust
pub type Rendering = Vec<(Shape, Style)>;
```
---

![A letter d tossed](image/d_tossed.png)

---

![A box; the way we instruct a painter](image/box.png)
![A flipped box](image/box_tossed.png)

---

```rust
pub fn toss_box(bx: &Box) -> Box {
    Box::new(
        bx.a.add(&bx.b.add(&bx.c).scale(&0.5)),
        bx.b.add(&bx.c).scale(&0.5),
        bx.c.sub(&bx.b).scale(&0.5),
    )
}
```

---

![Letter d above letter b](image/d_above_b.png)

---

![A box; the way we instruct a painter](image/box.png)
![A box above another box](image/box_above_box.png)

---

![Letter d besides letter b](image/d_beside_b.png)

---

![A box; the way we instruct a painter](image/box.png)
![A box besides another box](image/box_beside_box.png)

---

![A quartet of letters](image/quartet_of_d.png)

---

```rust
pub fn quartet<P, Q, R, S>(
    nw: Rc<P>,
    ne: Rc<Q>,
    sw: Rc<R>,
    se: Rc<S>,
) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
    R: Fn(&Bx) -> Rendering,
    S: Fn(&Bx) -> Rendering,
{
    above(
        beside(nw, ne),
        beside(sw, se)
    )
}
```

---

![A nonet of letters](image/nonet_of_d.png)

---

```rust
column(
    row(nw, nm, ne),
    row(mw, mm, me),
    row(sw, sm, se)
)
```

--

```rust
/// column
above_ratio(n, above(m, s), 1, 2)
```

``` rust
/// row
beside_ratio(w, beside(m, e), 1, 2)
```


---

![Letter d over letter d](image/d_over_b.png)

---

![A fish](image/fish.png)

---

![How to draw an owl](image/owl.jpg)

---

![T-tile](image/ttile.png)

---

```rust
let big = p.clone();
let top = flip(toss(p));
let right = turn(turn(turn(top.clone())));
over(big, over(top, right))
```

---

![U-tile](image/utile.png)

---

```rust
let top = flip(toss(p));
let upper_left = over(top.clone(), turn(top));
over(upper_left.clone(), turn(turn(upper_left)))

```

---

![Side](image/order_3_side.png)

---

![Side with an grid overlaid](image/order_3_side_grid.png)

---

```rust
Rc::new(move |bx: &Bx| {
    if n == 0 {
        let q = blank();
        q(bx)
    } else {
        let recurse = side(p.clone(), n - 1);
        let se = ttile(p.clone());
        let sw = turn(se.clone());
        let q = quartet(recurse.clone(), recurse, sw, se);
        q(bx)
    }
})
```

---

![Corner](image/order_3_corner.png)

---

![Corner with an grid overlaid](image/order_3_corner_grid.png)

---

```rust
Rc::new(move |bx: &Bx| {
    if n == 0 {
        let q = blank();
        q(bx)
    } else {
        let nw = corner(p.clone(), n - 1);
        let ne = side(p.clone(), n - 1);
        let sw = turn(ne.clone());
        let se = utile(p.clone());
        let q = quartet(nw, ne, sw, se);
        q(bx)
    }
})
```

---

![Square Limit](image/order_3_square_limit.png)

---

![Square Limit with an grid overlaid](image/order_3_square_limit_grid.png)

---

```rust
Rc::new(move |bx: &Bx| {
    if n == 0 {
        let q = blank();
        q(bx)
    } else {
        let mm = utile(p.clone());

        let nm = side(p.clone(), n);
        let mw = turn(nm.clone());
        let sm = turn(mw.clone());
        let me = turn(sm.clone());

        let nw = corner(p.clone(), n);
        let sw = turn(nw.clone());
        let se = turn(sw.clone());
        let ne = turn(se.clone());

        let q = nonet(nw, nm, ne, mw, mm, me, sw, sm, se);
        q(bx)
    }
})
```
---

# Functional Geometry

---

![Circle Limit III](image/Escher_Circle_Limit_III.jpg)