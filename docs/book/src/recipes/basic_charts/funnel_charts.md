# Funnel Charts

A funnel chart shows how a quantity narrows as it passes through a sequence of stages. It is a
*containment* form: each stage is understood to be a subset of the one above it, and the band
widths carry that claim — so it suits pipelines that genuinely nest, such as a conversion funnel.

Stages are fed **upstream-first**: plotly draws index 0 at the top, which is the opposite of a
plain category axis.

`text_info` takes a `+`-joined flaglist, so a band can show its own value alongside its conversion
from the previous stage (`"value+percent previous"`), the share of the first stage
(`"percent initial"`), or the share of the total (`"percent total"`).

For a sequence whose steps do *not* nest — independent totals, or contributions that can be
negative — see [Waterfall Charts](./waterfall_charts.md) instead.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::color::NamedColor;
use plotly::common::{Marker, Orientation};
use plotly::funnel::Connector as FunnelConnector;
use plotly::{Funnel, Plot};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Funnel
```rust,no_run
{{#include ../../../../../examples/basic_charts/src/main.rs:basic_funnel}}
```

{{#include ../../../../../examples/basic_charts/output/inline_basic_funnel.html}}
