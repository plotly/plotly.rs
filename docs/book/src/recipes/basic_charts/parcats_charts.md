# Parallel Categories

A parallel categories (`parcats`) chart visualizes multi-dimensional categorical
data as a set of parallel axes, one per category column, with ribbons flowing
between the categories. Like Sankey diagrams, it is domain-based rather than
cartesian — each [`ParcatsDimension`](https://docs.rs/plotly/latest/plotly/parcats/struct.ParcatsDimension.html)
defines one axis, and optional `counts` let you weight aggregated paths instead
of plotting one row per observation.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::color::NamedColor;
use plotly::common::Domain;
use plotly::layout::Layout;
use plotly::parcats::{
    ParcatsArrangement, ParcatsDimension, ParcatsHoverInfo, ParcatsHoverOn, ParcatsLine,
    ParcatsLineShape,
};
use plotly::{Parcats, Plot};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Parallel Categories
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_parcats}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_parcats.html}}


## Styled Parallel Categories
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:styled_parcats}}
```

{{#include ../../../../../examples/basic_charts/output/inline_styled_parcats.html}}
