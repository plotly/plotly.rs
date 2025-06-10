# Candlestick Charts

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{TickFormatStop, Title};
use plotly::layout::{Axis, RangeSelector, RangeSlider, SelectorButton, SelectorStep, StepMode};
use plotly::{Candlestick, Layout, Ohlc, Plot, Scatter};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Simple Candlestick Chart
```rust,no_run
{{#include ../../../../../examples/financial_charts/src/main.rs:simple_candlestick_chart}}
```

{{#include ../../../../../examples/financial_charts/output/inline_simple_candlestick_chart.html}}