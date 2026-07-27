# 2D Histograms

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{ColorScale, ColorScalePalette};
use plotly::histogram::{Bins, HistFunc, HistNorm};
use plotly::layout::{Axis, Layout};
use plotly::{Histogram2d, Plot};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic 2D Histogram
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:basic_histogram2d}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_basic_histogram2d.html}}


## Styled 2D Histogram
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:styled_histogram2d}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_styled_histogram2d.html}}


## 2D Histogram with Per-Sample Aggregation
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:histogram2d_aggregation}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_histogram2d_aggregation.html}}
