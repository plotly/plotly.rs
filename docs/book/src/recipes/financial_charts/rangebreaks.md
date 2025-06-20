# Rangebreaks

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::common::{Mode, Title};
use plotly::layout::{Axis, RangeBreak};
use plotly::{Layout, Plot, Scatter};
use chrono::{DateTime, Duration};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Series with Weekend and Holiday Gaps
```rust,no_run
{{#include ../../../../../examples/financial_charts/src/main.rs:series_with_gaps_for_weekends_and_holidays}}
```

{{#include ../../../../../examples/financial_charts/output/inline_series_with_gaps_for_weekends_and_holidays.html}}


## Hiding Weekend and Holiday Gaps with Rangebreaks
```rust,no_run
{{#include ../../../../../examples/financial_charts/src/main.rs:hiding_weekends_and_holidays_with_rangebreaks}}
```

{{#include ../../../../../examples/financial_charts/output/inline_hiding_weekends_and_holidays_with_rangebreaks.html}}


## Series with Non-Business Hours Gaps
```rust,no_run
{{#include ../../../../../examples/financial_charts/src/main.rs:series_with_non_business_hours_gaps}}
```

{{#include ../../../../../examples/financial_charts/output/inline_series_with_non_business_hours_gaps.html}}


## Hiding Non-Business Hours Gaps with Rangebreaks
```rust,no_run
{{#include ../../../../../examples/financial_charts/src/main.rs:hiding_non_business_hours_with_rangebreaks}}
```

{{#include ../../../../../examples/financial_charts/output/inline_hiding_non_business_hours_with_rangebreaks.html}} 
