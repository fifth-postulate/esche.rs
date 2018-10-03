# over
<img src="image/d_over_b.svg" alt="The letter d over the tossed letter d" width="400px" height="400px">

This is the last stop before we start drawing fishes. It feels a bit low-level
in comparison to quartet and nonet, but we can still make it in the comfort of
our `src/picture/mod.rs`.

The image that we show is that of the letter d that is drawn over a tossed
letter d. So our `over` function should take two pictures, draw one and draw the
other in the same box.

## Implementation.
Without giving the crux away, I would like to point out the definition of
`Rendering` again.

```rust
pub type Rendering = Vec<(Shape, Style)>;
```

It is nothing more than a vector of shape & style pairs. You can probably use
that to your advantage. If you have a hard time figuring out this primitive,
make sure to discuss it with your neighbor.

Once you get the solution show your work to your discussion partner by using it
in the `geometry` binary.
