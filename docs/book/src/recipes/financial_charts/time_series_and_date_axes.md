# Time Series and Date Axes

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

## Time Series Plot with Custom Date Range
```rust,no_run
{{#include ../../../../../examples/financial_charts/src/main.rs:time_series_plot_with_custom_date_range}}
```

{{#include ../../../../../examples/financial_charts/output/inline_time_series_plot_with_custom_date_range.html}}


## Time Series with Range Slider
```rust,no_run
{{#include ../../../../../examples/financial_charts/src/main.rs:time_series_with_range_slider}}
```

{{#include ../../../../../examples/financial_charts/output/inline_time_series_with_range_slider.html}}


## Time Series with Range Selector Buttons
```rust,no_run
{{#include ../../../../../examples/financial_charts/src/main.rs:time_series_with_range_selector_buttons}}
```

{{#include ../../../../../examples/financial_charts/output/inline_time_series_with_range_selector_buttons.html}}


## Customizing Tick Label Formatting by Zoom Level
```rust,no_run
{{#include ../../../../../examples/financial_charts/src/main.rs:customizing_tick_label_formatting_by_zoom_level}}
```

{{#include ../../../../../examples/financial_charts/output/inline_customizing_tick_label_formatting_by_zoom_level.html}}