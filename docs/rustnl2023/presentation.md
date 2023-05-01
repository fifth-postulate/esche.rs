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
![A turned box](image/box_flipped.png)
