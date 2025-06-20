# Themes

The complete source code for the following examples can also be found [here](https://github.com/plotly/plotly.rs/tree/main/examples/themes).

# Use different theme templates

Similar to [Plotly Python templates](https://plotly.com/python/templates/), plotly.rs provides several built-in themes that can be applied to your plots.

```rust,no_run
use plotly::{
    common::{Marker, Mode, Title},
    layout::{Layout, BuiltinTheme},
    Plot, Scatter,
};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Default Theme (Plotly)

{{#include ../../../../examples/themes/output/inline_gapminder_plotly.html}}

## Plotly White Theme

{{#include ../../../../examples/themes/output/inline_gapminder_plotly_white.html}}

## Plotly Dark Theme

{{#include ../../../../examples/themes/output/inline_gapminder_plotly_dark.html}}

## Seaborn Theme

{{#include ../../../../examples/themes/output/inline_gapminder_seaborn.html}}

## Matplotlib Theme

{{#include ../../../../examples/themes/output/inline_gapminder_matplotlib.html}}

## Plotnine Theme

{{#include ../../../../examples/themes/output/inline_gapminder_plotnine.html}}

## Available Themes

The following built-in themes are available in plotly.rs:

- `BuiltinTheme::Default` - Default Plotly theme
- `BuiltinTheme::PlotlyWhite` - Clean white background theme
- `BuiltinTheme::PlotlyDark` - Dark theme
- `BuiltinTheme::Seaborn` - Seaborn-style theme
- `BuiltinTheme::SeabornWhitegrid` - Seaborn with white grid
- `BuiltinTheme::SeabornDark` - Dark Seaborn theme
- `BuiltinTheme::Matplotlib` - Matplotlib-style theme
- `BuiltinTheme::Plotnine` - Plotnine-style theme

## Using Themes

To apply a theme to your plot, use the `template()` method on the layout:

```rust
let theme = BuiltinTheme::Seaborn;
let layout = Layout::new().template(theme.build());
plot.set_layout(layout);
```

The example above uses real Gapminder 2007 data showing the relationship between GDP per capita, life expectancy, and population size across different continents, with marker sizes representing population and colors representing continents.
