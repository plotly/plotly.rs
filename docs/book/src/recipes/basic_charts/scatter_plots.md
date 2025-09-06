# Scatter Plots

The following imports have been used to produce the plots below:

```rust,no_run
use ndarray::Array;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::{Bar, color::{NamedColor, Rgb, Rgba}, Plot, Scatter};
use rand_distr::{Distribution, Normal, Uniform};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Simple Scatter Plot
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:simple_scatter_plot}}
```

{{#include ../../../../../examples/basic_charts/output/inline_simple_scatter_plot.html}}


## Line and Scatter Plots
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:line_and_scatter_plots}}
```

{{#include ../../../../../examples/basic_charts/output/inline_line_and_scatter_plots.html}}

## Bubble Scatter Plots
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:bubble_scatter_plots}}
```

{{#include ../../../../../examples/basic_charts/output/inline_bubble_scatter_plots.html}}

## Polar Scatter Plots
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:polar_scatter_plot}}
```

{{#include ../../../../../examples/basic_charts/output/inline_polar_scatter_plot.html}}


## Data Labels Hover
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:data_labels_hover}}
```

{{#include ../../../../../examples/basic_charts/output/inline_data_labels_hover.html}}


## Data Labels on the Plot
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:data_labels_on_the_plot}}
```

{{#include ../../../../../examples/basic_charts/output/inline_data_labels_on_the_plot.html}}


## Colored and Styled Scatter Plot
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:colored_and_styled_scatter_plot}}
```

{{#include ../../../../../examples/basic_charts/output/inline_colored_and_styled_scatter_plot.html}}


## Large Data Sets
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:large_data_sets}}
```

{{#include ../../../../../examples/basic_charts/output/inline_large_data_sets.html}}
