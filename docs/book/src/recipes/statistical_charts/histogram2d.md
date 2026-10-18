# 2D Histograms

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{ColorScale, ColorScalePalette};
use plotly::histogram::{Bins, HistFunc, HistNorm};
use plotly::layout::{Axis, Layout};
use plotly::{Histogram2d, Histogram2dContour, Plot};
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


## 2D Histogram Contour
A `Histogram2dContour` bins the samples exactly like a `Histogram2d`, but draws the binned counts as
contour levels instead of heatmap cells. `n_contours` sets the maximum number of levels; pass a
`Contours` value to set them explicitly.

```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:histogram2d_contour}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_histogram2d_contour.html}}
