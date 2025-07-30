# Heatmaps

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{ColorScale, ColorScalePalette, Title};
use plotly::contour::Contours;
use plotly::{Contour, HeatMap, Layout, Plot};
use std::f64::consts::PI;
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Heatmap
```rust,no_run
{{#include ../../../../../examples/scientific_charts/src/main.rs:basic_heat_map}}
```

{{#include ../../../../../examples/scientific_charts/output/inline_basic_heat_map.html}}