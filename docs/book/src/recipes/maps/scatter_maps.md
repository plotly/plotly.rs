# Scatter Maps

Scatter map traces place markers or lines on geographic basemaps. Two trace types
are available:

- [`ScatterGeo`](https://docs.rs/plotly/latest/plotly/struct.ScatterGeo.html) —
  drawn on the built-in `geo` subplot
  ([`LayoutGeo`](https://docs.rs/plotly/latest/plotly/layout/struct.LayoutGeo.html)).
- [`ScatterMapbox`](https://docs.rs/plotly/latest/plotly/struct.ScatterMapbox.html) —
  drawn on a Mapbox-style basemap via
  [`Mapbox`](https://docs.rs/plotly/latest/plotly/layout/struct.Mapbox.html).

The following imports are used in the examples below:

```rust,no_run
use plotly::{
    common::{Line, Marker, Mode},
    layout::{
        Axis, Center, DragMode, GeoResolution, LayoutGeo, Mapbox, MapboxStyle, Projection,
        Rotation,
    },
    color::Rgb,
    Configuration, Layout, Plot, ScatterGeo, ScatterMapbox,
};
```

The `to_inline_html` method is used to produce the html plots displayed in this
page. The Mapbox example requires an internet connection for the basemap tiles.

## Scatter on a Mapbox basemap

```rust,no_run
{{#include ../../../../../examples/maps/src/main.rs:scatter_mapbox}}
```

{{#include ../../../../../examples/maps/output/inline_scatter_mapbox.html}}

## Lines on an orthographic geo subplot

The example below downloads contour data from the Plotly datasets repository. If
the fetch fails (for example during an offline build), the example is skipped with
a warning rather than panicking.

```rust,no_run
{{#include ../../../../../examples/maps/src/main.rs:scatter_geo}}
```

{{#include ../../../../../examples/maps/output/inline_scatter_geo.html}}
