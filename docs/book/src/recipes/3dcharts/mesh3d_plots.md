# Mesh3D Plots

A `Mesh3D` trace draws a triangulated 3D mesh from vertex coordinates and triangle
indices. An optional `intensity` array per vertex drives the color scale.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::ColorScalePalette;
use plotly::{Mesh3D, Plot};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Mesh3D Plot

```rust,no_run
{{#include ../../../../../examples/3d_charts/src/main.rs:mesh_3d_plot}}
```

{{#include ../../../../../examples/3d_charts/output/inline_mesh_3d_plot.html}}
