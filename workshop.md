# Workshop
All the files needed for this workshop

## starter
Contains the starter code you can work from.

## docs
A `mdbook` containing instructions. Find it [online](https://fifth-postulate.nl/esche.rs/) 
or serve it locally

```
cd docs; python -mSimpleHTTPServer 
```

## cargo_home 
We've prepared a directory containing all cargo dependencies, which will come in
handy if you're using this workshop package in a situation where internet
connections are spotty or non-existent.

To use this setup, set `CARGO_HOME` before proceeding to the location of the
cargo_home directory in this package.

On Linux/macOS:

```
export CARGO_HOME=/...where you've unpacked.../cargo_home
```

On Windows:

```
set CARGO_HOME=C:\...where you've unpacked...\cargo_home
```

Afterwards, cd to the appropriate directory and run cargo as usual:

```
cargo build ...
cargo run ...
```
