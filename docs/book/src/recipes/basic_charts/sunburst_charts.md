# Sunburst Charts

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::Orientation;
use plotly::sunburst::Leaf;
use plotly::treemap::BranchValues;
use plotly::{Plot, Sunburst};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Sunburst
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_sunburst}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_sunburst.html}}


## Styled Sunburst with Branch Values and Leaf Opacity
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:styled_sunburst}}
```

{{#include ../../../../../examples/basic_charts/output/inline_styled_sunburst.html}}
