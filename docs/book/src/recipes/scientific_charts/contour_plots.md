# Contour Plots

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{ColorScale, ColorScalePalette, Title};
use plotly::contour::Contours;
use plotly::{Contour, HeatMap, Layout, Plot};
use std::f64::consts::PI;
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Simple Contour Plot
```rust,no_run
{{#include ../../../../../examples/scientific_charts/src/main.rs:simple_contour_plot}}
```

{{#include ../../../../../examples/scientific_charts/out/simple_contour_plot.html}}


## Colorscale for Contour Plot
```rust,no_run
{{#include ../../../../../examples/scientific_charts/src/main.rs:colorscale_for_contour_plot}}
```

{{#include ../../../../../examples/scientific_charts/out/colorscale_for_contour_plot.html}}


## Customizing Size and Range of a Contour Plot Contours
```rust,no_run
{{#include ../../../../../examples/scientific_charts/src/main.rs:customizing_size_and_range_of_a_contour_plots_contours}}
```

{{#include ../../../../../examples/scientific_charts/out/customizing_size_and_range_of_a_contour_plots_contours.html}}


## Customizing Spacing Between X and Y Ticks
```rust,no_run
{{#include ../../../../../examples/scientific_charts/src/main.rs:customizing_spacing_between_x_and_y_ticks}}
```

{{#include ../../../../../examples/scientific_charts/out/customizing_spacing_between_x_and_y_ticks.html}}