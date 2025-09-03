# Sankey Diagrams

The following imports have been used to produce the plots below:

```rust,no_run
use ndarray::Array;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::Sankey;
use rand_distr::{Distribution, Normal, Uniform};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Constructing a basic Sankey diagram
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_sankey_diagram}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_sankey_diagram.html}}


## Skankey diagram with defined node position
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:custom_node_sankey_diagram}}
```

{{#include ../../../../../examples/basic_charts/output/inline_custom_node_sankey_diagram.html}}
