# `eschers`
`Rust` code exploring functional geometry.

## Prerequisites
`Rust` version should be `1.26` or higher.

## Motivation
The reason behind the `Rc<Fn(&Box) -> List<(Shape, Style)>` is to be able to
clone the closure.
