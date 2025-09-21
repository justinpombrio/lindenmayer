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

## Curves

Square space filling curves:

- `hilbert` -- Hilbert's classic square space filling curve.
- `moore` -- Moore's curve is very similar to the Hilbert curve but with the
  ends touching.
- `peano` -- Peano's curve. Stripey.
- `wunderlich` -- Wunderlich's first curve
- `wunderlich-3` -- Wunderlich's third curve
- `zigzag` -- A Wunderlich-like curve of my own devising.
- `zorder` -- the z-order curve. Not continuous.
- `sierpinski-curve` -- A fascinating curve. Consists entirely of quarter turns,
  but has eighth turn symmetries. I'd like to learn of more curves like this
  one.

Less than two dimensional curves:

- `sierpinski-triangle` -- Sierpinski's famous triangle. Much less interesting
  than his less well known square space filling curve, IMO. Approx 1.59
  dimensional.
- `koch` -- the Koch snowflake. Filled in for clarity. Approx 1.26 dimensional.
- `koch-90` -- a variant of the Koch snowflake using quarter turns instead of
  sixth turns.

Other curves:

- `gosper` -- Gosper's hexagonal space filling curve. The only simple natural
  one, I think.
- `dragon` -- the curve you get by folding a sheet of paper in half to a quarter
  turn, then folding its two straight segments in half at quarter turns, etc.
- `steeman` -- A space filling curve by Dieter K. Steemann, shown in Pintrest.
- `arioni` -- A space filling curve by J. Arioni, published in 2017.
- `s-curve` -- A curve of my own devising. It's very simple, so I wouldn't be
  surprised if I wasn't the first to find it.

Self-overlapping curves of my own devising, which look pretty but make me feel
shame because a curve shouldn't touch itself:

- `triangle`
- `fivefold`

Running `./save_curves.sh` will generate examples of most of the in `images/`.

## Extension

If you want to add another space filling curve or color scheme, look in
`src/main.rs`. They're all defined in there, and generally very eloquent!
