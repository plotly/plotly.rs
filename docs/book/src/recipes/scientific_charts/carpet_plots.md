# Carpet Plots

A `Carpet` trace defines a 2D coordinate system from two parameters `a` and `b`, mapped onto
cartesian `x`/`y` positions (when `x` is omitted, a *cheater plot* is drawn). Carpets are used to
show how a quantity depends on two independent variables.

`ScatterCarpet` and `ContourCarpet` traces are then drawn *on* a carpet: they take `a`/`b`
coordinates and refer to the carpet by its `carpet` id (or use the first carpet when no id is
given). The `a` and `b` axes are styled with `CarpetAxis`.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::carpet::{CarpetAxis, CarpetAxisTitle};
use plotly::common::{ColorScale, ColorScalePalette, Mode};
use plotly::contour::{Coloring, Contours};
use plotly::{Carpet, ContourCarpet, Layout, Plot, ScatterCarpet};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Scatter on a Carpet
```rust,no_run
{{#include ../../../../../examples/scientific_charts/src/main.rs:carpet_with_scatter}}
```

{{#include ../../../../../examples/scientific_charts/output/inline_carpet_with_scatter.html}}

## Contour Carpet
```rust,no_run
{{#include ../../../../../examples/scientific_charts/src/main.rs:contour_carpet}}
```

{{#include ../../../../../examples/scientific_charts/output/inline_contour_carpet.html}}
