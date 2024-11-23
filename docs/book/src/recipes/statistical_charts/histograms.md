# Histograms

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


## Basic Histogram
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:basic_histogram}}
```

{{#include ../../../../../examples/statistical_charts/out/basic_histogram.html}}


## Horizontal Histogram
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:horizontal_histogram}}
```

{{#include ../../../../../examples/statistical_charts/out/horizontal_histogram.html}}

## Overlaid Histogram
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:overlaid_histogram}}
```

{{#include ../../../../../examples/statistical_charts/out/overlaid_histogram.html}}


## Stacked Histograms
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:stacked_histograms}}
```

{{#include ../../../../../examples/statistical_charts/out/stacked_histograms.html}}


## Colored and Styled Histograms
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:colored_and_styled_histograms}}
```

{{#include ../../../../../examples/statistical_charts/out/colored_and_styled_histograms.html}}


## Cumulative Histogram
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:cumulative_histogram}}
```

{{#include ../../../../../examples/statistical_charts/out/cumulative_histogram.html}}


## Normalized Histogram
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:normalized_histogram}}
```

{{#include ../../../../../examples/statistical_charts/out/normalized_histogram.html}}


## Specify Binning Function
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:specify_binning_function}}
```

{{#include ../../../../../examples/statistical_charts/out/specify_binning_function.html}}