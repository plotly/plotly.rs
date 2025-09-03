# Pie Charts

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{Domain, Font, HoverInfo, Orientation};
use plotly::layout::{
        Annotation, Layout, LayoutGrid},
use plotly::layout::Layout;
use plotly::{Pie, Plot};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.


## Basic Pie Chart
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_pie_chart}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_pie_chart.html}}

```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_pie_chart_labels}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_pie_chart_labels.html}}

## Grouped Pie Chart
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:grouped_donout_pie_charts}}
```

{{#include ../../../../../examples/basic_charts/output/inline_grouped_donout_pie_charts.html}}

## Pie Chart Text Control
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:pie_chart_text_control}}
```

{{#include ../../../../../examples/basic_charts/output/inline_pie_chart_text_control.html}}