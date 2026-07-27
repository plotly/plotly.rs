# Table Charts

A `Table` trace renders structured data as an HTML table inside the plot. Unlike cartesian traces,
it does not use x/y axes — you supply a styled `Header` and `Cells` block.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::color::NamedColor;
use plotly::traces::table::{
    Align as TableAlign, Cells, Fill as TableFill, Font as TableFont, Header, Line as TableLine,
};
use plotly::{Plot, Table};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Table
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:table_chart}}
```

{{#include ../../../../../examples/basic_charts/output/inline_table_chart.html}}
