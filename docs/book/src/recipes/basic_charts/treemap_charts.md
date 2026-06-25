# Treemap Charts

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::treemap::{BranchValues, Packing, PathBar, Side, Tiling};
use plotly::{Plot, Treemap};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Treemap
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_treemap}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_treemap.html}}


## Styled Treemap with Tiling and Path Bar
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:styled_treemap}}
```

{{#include ../../../../../examples/basic_charts/output/inline_styled_treemap.html}}
