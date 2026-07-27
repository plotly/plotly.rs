# Density Maps

A [`DensityMapbox`](https://docs.rs/plotly/latest/plotly/struct.DensityMapbox.html)
trace renders a kernel density estimate as a heatmap layer on a Mapbox-style
basemap.

The following imports have been used to produce the plots below:

```rust,no_run
use plotly::{
    layout::{Center, DragMode, Mapbox, MapboxStyle},
    Configuration, DensityMapbox, Layout, Plot,
};
```

The `to_inline_html` method is used to produce the html plot displayed in this
page. The rendered map requires an internet connection for the basemap tiles.

## Basic Density Mapbox

```rust,no_run
{{#include ../../../../../examples/maps/src/main.rs:density_mapbox}}
```

{{#include ../../../../../examples/maps/output/inline_density_mapbox.html}}
