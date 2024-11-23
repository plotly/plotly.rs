# Shapes

The following imports have been used to produce the plots below:

```rust,no_run
use ndarray::Array;
use plotly::common::{
    Fill, Font, Mode,
};
use plotly::layout::{
    Axis, GridPattern, Layout, LayoutGrid, Margin, Shape, ShapeLayer, ShapeLine,
    ShapeType,
};
use plotly::{Bar, color::NamedColor, Plot, Scatter};
use rand::thread_rng;
use rand_distr::{Distribution, Normal};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.


## Filled Area Chart
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:filled_area_chart}}
```

{{#include ../../../../examples/shapes/out/filled_area_chart.html}}


## Vertical and Horizontal Lines Positioned Relative to Axes
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:vertical_and_horizontal_lines_positioned_relative_to_axes}}
```

{{#include ../../../../examples/shapes/out/vertical_and_horizontal_lines_positioned_relative_to_axes.html}}


## Lines Positioned Relative to the Plot and to the Axes
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:lines_positioned_relative_to_the_plot_and_to_the_axes}}
```

{{#include ../../../../examples/shapes/out/lines_positioned_relative_to_the_plot_and_to_the_axes.html}}


## Creating Tangent Lines with Shapes
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:creating_tangent_lines_with_shapes}}
```

{{#include ../../../../examples/shapes/out/creating_tangent_lines_with_shapes.html}}


## Rectangles Positioned Relative to the Axes
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:rectangles_positioned_relative_to_the_axes}}
```

{{#include ../../../../examples/shapes/out/rectangles_positioned_relative_to_the_axes.html}}


## Rectangle Positioned Relative to the Plot and to the Axes
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:rectangle_positioned_relative_to_the_plot_and_to_the_axes}}
```

{{#include ../../../../examples/shapes/out/rectangle_positioned_relative_to_the_plot_and_to_the_axes.html}}


## Highlighting Time Series Regions with Rectangle Shapes
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:highlighting_time_series_regions_with_rectangle_shapes}}
```

{{#include ../../../../examples/shapes/out/highlighting_time_series_regions_with_rectangle_shapes.html}}


## Circles Positioned Relative to the Axes
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:circles_positioned_relative_to_the_axes}}
```

{{#include ../../../../examples/shapes/out/circles_positioned_relative_to_the_axes.html}}


## Highlighting Clusters of Scatter Points with Circle Shapes
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:highlighting_clusters_of_scatter_points_with_circle_shapes}}
```

{{#include ../../../../examples/shapes/out/highlighting_clusters_of_scatter_points_with_circle_shapes.html}}


## Venn Diagram with Circle Shapes
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:venn_diagram_with_circle_shapes}}
```

{{#include ../../../../examples/shapes/out/venn_diagram_with_circle_shapes.html}}


## Adding Shapes to Subplots
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:adding_shapes_to_subplots}}
```

{{#include ../../../../examples/shapes/out/adding_shapes_to_subplots.html}}


## SVG Paths
```rust,no_run
{{#include ../../../../examples/shapes/src/main.rs:svg_paths}}
```

{{#include ../../../../examples/shapes/out/svg_paths.html}}
