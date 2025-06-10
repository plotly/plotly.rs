# Bar Charts

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


## Basic Bar Chart
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_bar_chart}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_bar_chart.html}}

## Grouped Bar Chart
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:grouped_bar_chart}}
```

{{#include ../../../../../examples/basic_charts/output/inline_grouped_bar_chart.html}}

## Stacked Bar Chart
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:stacked_bar_chart}}
```

{{#include ../../../../../examples/basic_charts/output/inline_stacked_bar_chart.html}}
