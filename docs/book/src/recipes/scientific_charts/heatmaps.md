# Heatmaps

The following imports have been used to produce the plots below:

```rust
use plotly::common::{ColorScale, ColorScalePalette, Title};
use plotly::contour::Contours;
use plotly::{Contour, HeatMap, Layout, Plot};
use std::f64::consts::PI;
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Heatmap
```rust
fn basic_heat_map(show: bool) {
    let z = vec![vec![1, 20, 30], vec![20, 1, 60], vec![30, 60, 1]];
    let trace = HeatMap::new_z(z);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    if show {
        plot.show().unwrap();
    }
    println!("{}", plot.to_inline_html(Some("basic_heat_map")).unwrap());
}
```
<div id="basic_heat_map" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("basic_heat_map")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"heatmap","x":null,"y":null,"z":[[1,20,30],[20,1,60],[30,60,1]]};
var data = [trace_0];
var layout = {};
        Plotly.newPlot('basic_heat_map', data, layout, {"responsive": true});
    };
</script>