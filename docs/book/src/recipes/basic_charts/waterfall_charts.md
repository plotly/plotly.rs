# Waterfall Charts

A waterfall chart shows how a running total is built up from a starting value and a
sequence of signed contributions. Each bar's role is set by its `Measure`:

- `Measure::Absolute` resets the running total and draws a bar from zero to it,
- `Measure::Relative` adds a signed delta, drawn as a floating bar,
- `Measure::Total` draws the running total accumulated so far.

The value supplied for a `Total` bar is ignored — plotly.js re-derives it — but the slot
must still be present, because the label, value and `measure` arrays are read positionally.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::color::NamedColor;
use plotly::waterfall::{Marker as WaterfallMarker, Measure, MeasureStyle};
use plotly::{Plot, Waterfall};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Waterfall
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_waterfall}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_waterfall.html}}
