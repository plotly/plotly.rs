# Box Plots

The following imports have been used to produce the plots below:

```rust,no_run
use ndarray::Array;
use plotly::box_plot::{BoxMean, BoxPoints};
use plotly::common::{ErrorData, ErrorType, Line, Marker, Mode, Orientation, Title};
use plotly::histogram::{Bins, Cumulative, HistFunc, HistNorm};
use plotly::layout::{Axis, BarMode, BoxMode, Layout, Margin};
use plotly::{Bar, BoxPlot, Histogram, color::{NamedColor, Rgb, Rgba}, Scatter};
use rand_distr::{Distribution, Normal, Uniform};

```

The `to_inline_html` method is used to produce the html plot displayed in this page.


## Basic Box Plot
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:basic_box_plot}}
```

{{#include ../../../../../examples/statistical_charts/out/basic_box_plot.html}}


## Box Plot that Displays the Underlying Data
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:box_plot_that_displays_the_underlying_data}}
```

{{#include ../../../../../examples/statistical_charts/out/box_plot_that_displays_the_underlying_data.html}}


## Horizontal Box Plot
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:horizontal_box_plot}}
```

{{#include ../../../../../examples/statistical_charts/out/horizontal_box_plot.html}}


## Grouped Box Plot
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:grouped_box_plot}}
```

{{#include ../../../../../examples/statistical_charts/out/grouped_box_plot.html}}


## Box Plot Styling Outliers
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:box_plot_styling_outliers}}
```

{{#include ../../../../../examples/statistical_charts/out/box_plot_styling_outliers.html}}


## Box Plot Styling Mean and Standard Deviation
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:box_plot_styling_mean_and_standard_deviation}}
```

{{#include ../../../../../examples/statistical_charts/out/box_plot_styling_mean_and_standard_deviation.html}}


## Grouped Horizontal Box Plot
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:grouped_horizontal_box_plot}}
```

{{#include ../../../../../examples/statistical_charts/out/grouped_horizontal_box_plot.html}}


## Fully Styled Box Plot
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:fully_styled_box_plot}}
```

{{#include ../../../../../examples/statistical_charts/out/fully_styled_box_plot.html}}