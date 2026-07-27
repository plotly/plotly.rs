# Icicle Charts

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::color::NamedColor;
use plotly::common::{Line, Orientation};
use plotly::icicle::{BranchValues, Leaf, Marker, PathBar, Side, Tiling};
use plotly::{Icicle, Plot};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Icicle
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_icicle}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_icicle.html}}


## Styled Icicle with Tiling and Path Bar
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:styled_icicle}}
```

{{#include ../../../../../examples/basic_charts/output/inline_styled_icicle.html}}
