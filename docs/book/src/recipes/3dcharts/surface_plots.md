# Surface Plots

A `Surface` trace renders a 3D surface from a matrix of z-values. Optional `x` and
`y` vectors define the grid coordinates; when omitted, indices are used.

The following imports have been used to produce the plots below:

```rust,no_run
use ndarray::Array;
use plotly::{Plot, Surface};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Surface Plot

```rust,no_run
{{#include ../../../../../examples/3d_charts/src/main.rs:surface_plot}}
```

{{#include ../../../../../examples/3d_charts/output/inline_surface_plot.html}}
