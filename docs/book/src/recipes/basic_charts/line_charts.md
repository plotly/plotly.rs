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

{{#include ../../../../../examples/basic_charts/out/adding_names_to_line_and_scatter_plot.html}}


## Line and Scatter Styling
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:line_and_scatter_styling}}
```

{{#include ../../../../../examples/basic_charts/out/line_and_scatter_styling.html}}

## Styling Line Plot
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:styling_line_plot}}
```

{{#include ../../../../../examples/basic_charts/out/styling_line_plot.html}}

## Line Shape Options for Interpolation
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:line_shape_options_for_interpolation}}
```

{{#include ../../../../../examples/basic_charts/out/line_shape_options_for_interpolation.html}}

## Line Dash
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:line_dash}}
```

{{#include ../../../../../examples/basic_charts/out/line_dash.html}}

## Filled Lines
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:filled_lines}}
```

{{#include ../../../../../examples/basic_charts/out/filled_lines.html}}