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

![A letter d flipped](image/d_flipped.png)

---

![A box; the way we instruct a painter](image/box.png)
![A flipped box](image/box_flipped.png)

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

![A nonet of letters](image/nonet_of_d.png)

---

![Letter d over letter d](image/d_over_b.png)

---

![A fish](image/fish.png)

---

![How to draw an owl](image/owl.jpg)

---

![T-tile](image/ttile.png)

---

![U-tile](image/utile.png)

---

![Side](image/order_3_side.png)

---

![Side with an grid overlaid](image/order_3_side_grid.png)

---

![Corner](image/order_3_corner.png)

---

![Corner with an grid overlaid](image/order_3_corner_grid.png)

---

![Square Limit](image/order_3_square_limit.png)

---

![Square Limit with an grid overlaid](image/order_3_square_limit_grid.png)



