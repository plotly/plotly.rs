# Multiple Axes

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{Font, AxisSide, Title};
use plotly::layout::{Axis, GridPattern, Layout, LayoutGrid, Legend, RowOrder};
use plotly::{Plot, Rgb, Scatter};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Two Y Axes
```rust,no_run
{{#include ../../../../../examples/subplots/src/main.rs:two_y_axes}}
```

{{#include ../../../../../examples/subplots/output/inline_two_y_axes.html}}


## Multiple Axes
```rust,no_run
{{#include ../../../../../examples/subplots/src/main.rs:multiple_axes}}
```

{{#include ../../../../../examples/subplots/output/inline_multiple_axes.html}}
