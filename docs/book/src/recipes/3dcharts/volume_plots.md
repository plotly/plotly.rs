# Isosurface and Volume Plots

`IsoSurface` and `Volume` traces both visualize a scalar `value` sampled on a 3D `(x, y, z)` grid.
An isosurface draws a few opaque surfaces of constant value between `iso_min` and `iso_max`
(the number is set by `IsoSurfaceSurface::count`), while a volume rendering stacks many
semi-transparent surfaces — use a low `opacity` and an `OpacityScale` to see inside the volume.

Both traces share the `Caps`, `Slices`, `SpaceFrame` and `IsoSurfaceSurface` settings from
`plotly::isosurface`. The grid points are given flattened, with x varying fastest.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{ColorScale, ColorScalePalette};
use plotly::isosurface::{Cap, Caps, IsoSurfaceSurface};
use plotly::layout::Layout;
use plotly::volume::OpacityScale;
use plotly::{IsoSurface, Plot, Volume};
```

The examples on this page build their grid with the following helper:

```rust,no_run
{{#include ../../../../../examples/3d_charts/src/main.rs:grid_3d}}
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Isosurface Plot
```rust,no_run
{{#include ../../../../../examples/3d_charts/src/main.rs:isosurface_plot}}
```

{{#include ../../../../../examples/3d_charts/output/inline_isosurface_plot.html}}

## Volume Plot
```rust,no_run
{{#include ../../../../../examples/3d_charts/src/main.rs:volume_plot}}
```

{{#include ../../../../../examples/3d_charts/output/inline_volume_plot.html}}
