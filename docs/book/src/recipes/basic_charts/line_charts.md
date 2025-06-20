# Line Charts

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


## Adding Names to Line and Scatter Plot
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:adding_names_to_line_and_scatter_plot}}
```

{{#include ../../../../../examples/basic_charts/output/inline_adding_names_to_line_and_scatter_plot.html}}


## Line and Scatter Styling
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:line_and_scatter_styling}}
```

{{#include ../../../../../examples/basic_charts/output/inline_line_and_scatter_styling.html}}

## Styling Line Plot
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:styling_line_plot}}
```

{{#include ../../../../../examples/basic_charts/output/inline_styling_line_plot.html}}

## Line Shape Options for Interpolation
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:line_shape_options_for_interpolation}}
```

{{#include ../../../../../examples/basic_charts/output/inline_line_shape_options_for_interpolation.html}}

## Line Dash
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:line_dash}}
```

{{#include ../../../../../examples/basic_charts/output/inline_line_dash.html}}

## Filled Lines
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:filled_lines}}
```

{{#include ../../../../../examples/basic_charts/output/inline_filled_lines.html}}

## Setting Lower or Upper Bounds on Axis
This example demonstrates how to set partial axis ranges using both the new `AxisRange` API and the backward-compatible vector syntax. The x-axis uses both the new `AxisRange::upper()` method and the traditional `vec![None, Some(value)]` syntax to set only an upper bound, while the y-axis uses only the `vec![Some(value), None]` syntax to set a lower bound.

```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:set_lower_or_upper_bound_on_axis}}
```

{{#include ../../../../../examples/basic_charts/output/inline_set_lower_or_upper_bound_on_axis.html}}