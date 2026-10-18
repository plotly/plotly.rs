# Vector Field Plots

A `Cone` trace draws a 3D vector field as cones: each cone sits at an `(x, y, z)` position and
points along the `(u, v, w)` vector, colored by the vector norm. `anchor` sets which part of the
cone sits on the position, and `size_mode`/`size_ref` control how the vector norm maps to cone size.

A `StreamTube` trace follows the flow of the same kind of field: tubes start from the `starts`
positions (or the lowest-`x` plane) and their diameter reflects the local divergence of the field.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{ColorBar, ColorScale, ColorScalePalette};
use plotly::layout::{Camera, Layout, LayoutScene};
use plotly::stream_tube::Starts;
use plotly::{Cone, Plot, StreamTube};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Cone Plot
```rust,no_run
{{#include ../../../../../examples/3d_charts/src/main.rs:cone_plot}}
```

{{#include ../../../../../examples/3d_charts/output/inline_cone_plot.html}}

## Streamtube Plot
```rust,no_run
{{#include ../../../../../examples/3d_charts/src/main.rs:streamtube_plot}}
```

{{#include ../../../../../examples/3d_charts/output/inline_streamtube_plot.html}}
