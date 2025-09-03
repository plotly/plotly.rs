# Error Bars

The following imports have been used to produce the plots below:

```rust,no_run
use ndarray::Array;
use plotly::box_plot::{BoxMean, BoxPoints};
use plotly::common::{ErrorData, ErrorType, Line, Marker, Mode, Orientation, Title};
use plotly::histogram::{Bins, Cumulative, HistFunc, HistNorm};
use plotly::layout::{Axis, BarMode, BoxMode, Layout, Margin};
use plotly::{Bar, BoxPlot, Histogram, Plot, color::{NamedColor, Rgb, Rgba}, Scatter};
use rand_distr::{Distribution, Normal, Uniform};

```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Symmetric Error Bars
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:basic_symmetric_error_bars}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_basic_symmetric_error_bars.html}}

## Asymmetric Error Bars
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:asymmetric_error_bars}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_asymmetric_error_bars.html}}

## Error Bars as a Percentage of the Y Value
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:error_bars_as_a_percentage_of_the_y_value}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_error_bars_as_a_percentage_of_the_y_value.html}}


## Asymmetric Error Bars with a Constant Offset
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:asymmetric_error_bars_with_a_constant_offset}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_asymmetric_error_bars_with_a_constant_offset.html}}


## Horizontal Error Bars
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:horizontal_error_bars}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_horizontal_error_bars.html}}


## Bar Chart with Error Bars
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:bar_chart_with_error_bars}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_bar_chart_with_error_bars.html}}


## Colored and Styled Error Bars
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:colored_and_styled_error_bars}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_colored_and_styled_error_bars.html}}