# Scatter 3D Charts

The following imports have been used to produce the plots below:

```rust,no_run
use ndarray::Array;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::Sankey;
use rand_distr::{Distribution, Normal, Uniform};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Constructing a basic Scatter 3D plot
```rust,no_run
{{#include ../../../../../examples/3d_charts/src/main.rs:simple_scatter3d_plot}}
```

{{#include ../../../../../examples/3d_charts/out/simple_scatter3d_plot.html}}