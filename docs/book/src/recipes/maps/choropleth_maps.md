# Choropleth Maps

Choropleth maps color geographic regions (countries, states, custom GeoJSON
areas) according to a data value. Two trace types are available:

- [`Choropleth`](https://docs.rs/plotly/latest/plotly/struct.Choropleth.html) —
  drawn on the built-in `geo` subplot
  ([`LayoutGeo`](https://docs.rs/plotly/latest/plotly/layout/struct.LayoutGeo.html)).
  Regions are matched by `location_mode` (ISO-3 codes, USA state codes, country
  names, or a GeoJSON id).
- [`ChoroplethMap`](https://docs.rs/plotly/latest/plotly/struct.ChoroplethMap.html) —
  drawn on the MapLibre `map` subplot
  ([`LayoutMap`](https://docs.rs/plotly/latest/plotly/layout/struct.LayoutMap.html)).
  Regions are always matched against a GeoJSON feature collection via
  `feature_id_key`.

The following imports are used in the examples below:

```rust,no_run
use plotly::{
    choropleth::{LocationMode, Marker as ChoroplethMarker},
    color::Rgb,
    common::{ColorBar, ColorScale, ColorScalePalette, Line},
    layout::{Center, DragMode, GeoResolution, LayoutGeo, LayoutMap, MapStyle},
    Choropleth, ChoroplethMap, Configuration, Layout, Plot,
};
```

The `to_inline_html` method is used to produce the html plots displayed in this
page. The rendered maps require an internet connection (the MapLibre basemap and,
for the second example, the remote GeoJSON are fetched in the browser).

## Choropleth on a geo subplot

```rust,no_run
{{#include ../../../../../examples/maps/src/main.rs:choropleth}}
```

{{#include ../../../../../examples/maps/output/inline_choropleth.html}}

## Choropleth on a MapLibre map subplot

```rust,no_run
{{#include ../../../../../examples/maps/src/main.rs:choropleth_map}}
```

{{#include ../../../../../examples/maps/output/inline_choropleth_map.html}}
