# Ternary Plots

A `ScatterTernary` trace plots three-component mixtures on a triangle: each point is given by its
`a`, `b` and `c` components, which are normalized to add up to the subplot's `sum`. The triangle
and its three axes are configured with `LayoutTernary` and `TernaryAxis`.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{Marker, Mode};
use plotly::layout::{Layout, LayoutTernary, TernaryAxis};
use plotly::{Plot, ScatterTernary};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Ternary Scatter Plot
```rust,no_run
{{#include ../../../../../examples/scientific_charts/src/main.rs:ternary_plot}}
```

{{#include ../../../../../examples/scientific_charts/output/inline_ternary_plot.html}}
