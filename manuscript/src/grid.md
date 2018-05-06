# Grid method
During the workshop we will be relying on a technique well know to painters. It
is called the [_grid method_](https://www.art-is-fun.com/grid-method).

Let's explain this method with our t**Rust**ed logo.

<img src="image/rust-logo-blk.svg" alt="The Rust logo" width="400px" height="400px">

The first thing we do to our logo is overlay a grid.

<img src="image/grid-logo.svg" alt="A grid over the Rust logo" width="400px" height="400px">

Next we draw a second grid, similar to the first.

<div class="reference">
  <img src="image/grid-logo.svg" alt="A grid over the Rust logo" width="400px" height="400px">
  <img src="image/grid.svg" alt="A single grid" width="400px" height="400px">
</div>

Now we focus on a single cell in the original grid, and accurately copy it in
the corresponding grid in the target grid. Going from top to bottom and from
left to right, our first cell is empty. The second cell has a single tooth of
the gear. Let's copy it over.

<div class="reference">
  <img src="image/grid-logo.svg" alt="A grid over the Rust logo" width="400px" height="400px">
  <img src="image/grid-logo-copy01.svg" alt="One cell copied of the Rust logo" width="400px" height="400px">
</div>

The following cell has some more teeth.

<div class="reference">
  <img src="image/grid-logo.svg" alt="A grid over the Rust logo" width="400px" height="400px">
  <img src="image/grid-logo-copy02.svg" alt="Two cells copied of the Rust logo" width="400px" height="400px">
</div>

By copying each cell one by one, the original logo is recreated.

<div class="reference">
  <img src="image/grid-logo.svg" alt="A grid over the Rust logo" width="400px" height="400px">
  <img src="image/grid-logo-copy03.svg" alt="Three cells copied of the Rust logo" width="400px" height="400px">
</div>

until the original is completely copied.


<div class="reference">
  <img src="image/grid-logo.svg" alt="A grid over the Rust logo" width="400px" height="400px">
  <img src="image/grid-logo.svg" alt="The Rust logo copied by the grid method" width="400px" height="400px">
</div>

## Variations
Once we understand the basics of the grid method, we can start to look for
variations. Nobody is restricting you to draw the target grid differently.

### Scale
We could scale the grid.

<div class="reference">
  <img src="image/grid-logo.svg" alt="A grid over the Rust logo" width="400px" height="400px">
  <img src="image/grid-logo-scaled.svg" alt="The Rust logo scaled by the grid method" width="400px" height="400px">
</div>

### Non-uniform scale
We could scale the grid non-uniformly.

<div class="reference">
  <img src="image/grid-logo.svg" alt="A grid over the Rust logo" width="400px" height="400px">
  <img src="image/grid-logo-non-uniformly-scaled.svg" alt="The Rust logo scaled by the grid method" width="400px" height="400px">
</div>

### Shearing
We could shear the grid.

<div class="reference">
  <img src="image/grid-logo.svg" alt="A grid over the Rust logo" width="400px" height="400px">
  <img src="image/grid-logo-shear.svg" alt="The Rust logo scaled by the grid method" width="400px" height="400px">
</div>

### Combination
Or we could combine various transformation and apply that to the grid.

<div class="reference">
  <img src="image/grid-logo.svg" alt="A grid over the Rust logo" width="400px" height="400px">
  <img src="image/grid-logo-combination.svg" alt="The Rust logo scaled by the grid method" width="400px" height="400px">
</div>

Because we will make extensive use of the grid method, and want to easily
manipulate the grid, we will introduce an _box_ abstraction.

