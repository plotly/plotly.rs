# Polar Bar Charts

A `BarPolar` trace draws bars on a polar subplot: `r` sets the radial length of each bar and
`theta` its angular position. Stacking several traces gives a *wind rose*. Unlike cartesian bars,
the stacking mode and the gap between bars are set per polar subplot with
`LayoutPolar::bar_mode` and `LayoutPolar::bar_gap`.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::color::NamedColor;
use plotly::common::Marker;
use plotly::layout::{AngularAxis, Layout, LayoutPolar, PolarBarMode, PolarDirection};
use plotly::{BarPolar, Plot};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Wind Rose
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:wind_rose}}
```

{{#include ../../../../../examples/basic_charts/output/inline_wind_rose.html}}
