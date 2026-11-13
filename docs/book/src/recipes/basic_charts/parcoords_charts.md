# Parallel Coordinates

A parallel coordinates (`parcoords`) chart draws each row of numeric data as a polyline across a
set of vertical axes, one per `Dimension`. Lines can be colored by a value through
`ParcoordsLine`, and a `ConstraintRange` pre-selects (brushes) an interval on an axis — the
same selection users can make interactively by dragging along an axis.

For categorical data, see [Parallel Categories](./parcats_charts.md).

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{ColorScale, ColorScalePalette};
use plotly::layout::Layout;
use plotly::parcoords::{ConstraintRange, Dimension, ParcoordsLine};
use plotly::{Parcoords, Plot};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Parallel Coordinates
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_parcoords}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_parcoords.html}}
