# Indicator Charts

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::color::NamedColor;
use plotly::common::{Domain, Line};
use plotly::layout::Layout;
use plotly::traces::indicator::{
    Align, Delta, DeltaPosition, Direction, Gauge, GaugeAxis, GaugeBar, GaugeShape,
    IndicatorTitle, Mode, Number, Step, Threshold,
};
use plotly::{Indicator, Plot};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Indicator with Number and Delta
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_indicator}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_indicator.html}}


## Indicator Gauge
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:indicator_gauge}}
```

{{#include ../../../../../examples/basic_charts/output/inline_indicator_gauge.html}}
