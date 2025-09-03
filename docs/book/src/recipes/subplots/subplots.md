# Subplots

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{Font, Side, Title};
use plotly::layout::{Axis, GridPattern, Layout, LayoutGrid, Legend, RowOrder};
use plotly::{Plot, Rgb, Scatter};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.


## Simple Subplot
```rust,no_run
{{#include ../../../../../examples/subplots/src/main.rs:simple_subplot}}
```

{{#include ../../../../../examples/subplots/output/inline_simple_subplot.html}}


## Subplots with Multiple Traces
```rust,no_run
{{#include ../../../../../examples/subplots/src/main.rs:subplots_with_multiple_traces}}
```

{{#include ../../../../../examples/subplots/output/inline_subplots_with_multiple_traces.html}}


## Custom Sized Subplot
```rust,no_run
{{#include ../../../../../examples/subplots/src/main.rs:custom_sized_subplot}}
```

{{#include ../../../../../examples/subplots/output/inline_custom_sized_subplot.html}}


## Multiple Subplots
```rust,no_run
{{#include ../../../../../examples/subplots/src/main.rs:multiple_subplots}}
```

{{#include ../../../../../examples/subplots/output/inline_multiple_subplots.html}}


## Stacked Subplots
```rust,no_run
{{#include ../../../../../examples/subplots/src/main.rs:stacked_subplots}}
```

{{#include ../../../../../examples/subplots/output/inline_stacked_subplots.html}}


## Stacked Subplots with Shared X Axis
```rust,no_run
{{#include ../../../../../examples/subplots/src/main.rs:stacked_subplots_with_shared_x_axis}}
```

{{#include ../../../../../examples/subplots/output/inline_stacked_subplots_with_shared_x_axis.html}}


## Multiple Custom Sized Subplots
```rust,no_run
{{#include ../../../../../examples/subplots/src/main.rs:multiple_custom_sized_subplots}}
```

{{#include ../../../../../examples/subplots/output/inline_multiple_custom_sized_subplots.html}}