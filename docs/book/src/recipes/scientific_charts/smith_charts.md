# Smith Charts

A `ScatterSmith` trace plots complex values — typically normalized impedances in RF engineering —
on a Smith chart, given by their `real` and `imag` parts. The chart's real and imaginary axes are
configured with `LayoutSmith`, `RealAxis` and `ImaginaryAxis`.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::Mode;
use plotly::layout::{Layout, LayoutSmith};
use plotly::{Plot, ScatterSmith};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Smith Chart
```rust,no_run
{{#include ../../../../../examples/scientific_charts/src/main.rs:smith_chart}}
```

{{#include ../../../../../examples/scientific_charts/output/inline_smith_chart.html}}
