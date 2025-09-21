# Lindenmayer

Draw space filling curves defined by Lindenmayer systems.

## Usage

To draw N iterations of a given CURVE:
      
```
cargo run CURVE N
```

The most basic options you'll want are:

- `--bg` sets the background color.
- `-c COLOR_SCALE` sets the color scale the curve. The names are cryptic; try
  them out to see what they look like. Most are parametric functions over OKLAB
  space. The ones beginning with `cet-` are CET perceptually uniform color maps
  (https://colorcet.com/). The color scale `h` is different: it maps points on
  the curve onto a 3d Hilbert curve in OKLAB color space.
- `-t` sets the curve thickness, where 1.0 is thick enough to touch itself.
  Using `-t 0` will draw individual pixels instead of line segments; if the
  curve is sufficiently dense you should switch to this method of drawing it.

To see the full set of options, as well as the curves and color scales
available, run `cargo run -- --help`.

## Extension

If you want to add another space filling curve or color scheme, look in
`src/main.rs`. They're all defined in there, and generally very eloquent!
