# Violin Plots

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{Line, Orientation};
use plotly::layout::{Layout, ViolinMode};
use plotly::violin::{MeanLine, ViolinBox, ViolinPoints, ViolinSide};
use plotly::{color::NamedColor, Plot, Violin};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.


## Basic Violin Plot
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:basic_violin_plot}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_basic_violin_plot.html}}


## Horizontal Violin Plot
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:horizontal_violin_plot}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_horizontal_violin_plot.html}}


## Split Violin Plot
```rust,no_run
{{#include ../../../../../examples/statistical_charts/src/main.rs:split_violin_plot}}
```

{{#include ../../../../../examples/statistical_charts/output/inline_split_violin_plot.html}}
