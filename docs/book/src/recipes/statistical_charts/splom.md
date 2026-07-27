# SPLOM

A SPLOM (scatter plot matrix) visualizes multivariate data as a grid of scatter
subplots — one cell for each pair of dimensions. Each column of your dataset
becomes a [`SplomDimension`](https://docs.rs/plotly/latest/plotly/splom/struct.SplomDimension.html),
and Plotly.js lays out the axis grid automatically, so you do not assign explicit
`xaxis`/`yaxis` ids as you would for cartesian traces.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::color::NamedColor;
use plotly::common::Marker;
use plotly::layout::Layout;
use plotly::splom::{SplomDiagonal, SplomDimension};
use plotly::{Plot, Splom};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic SPLOM
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:basic_splom}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_basic_splom.html}}


## Styled SPLOM
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:styled_splom}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_styled_splom.html}}
