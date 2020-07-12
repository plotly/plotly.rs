# Bar Charts

The following imports have been used to produce the plots below:

```rust
use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::{Bar, NamedColor, Plot, Rgb, Rgba, Scatter};
use rand_distr::{Distribution, Normal, Uniform};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.


## Basic Bar Chart
```rust 
fn basic_bar_chart(show: bool) {
    let animals = vec!["giraffes", "orangutans", "monkeys"];
    let t = Bar::new(animals, vec![20, 14, 23]);
    let mut plot = Plot::new();
    plot.add_trace(t);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("basic_bar_chart")));
}

```
<div id="basic_bar_chart" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("basic_bar_chart")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"x":["giraffes","orangutans","monkeys"],"y":[20,14,23],"type":"bar"};
var data = [trace_0];
var layout = {};
        Plotly.newPlot('basic_bar_chart', data, layout, {"responsive": true});
    };
</script>

## Grouped Bar Chart
```rust 
fn grouped_bar_chart(show: bool) {
    let animals1 = vec!["giraffes", "orangutans", "monkeys"];
    let trace1 = Bar::new(animals1, vec![20, 14, 23]).name("SF Zoo");

    let animals2 = vec!["giraffes", "orangutans", "monkeys"];
    let trace2 = Bar::new(animals2, vec![12, 18, 29]).name("LA Zoo");

    let layout = Layout::new().bar_mode(BarMode::Group);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("grouped_bar_chart")));
}
```
<div id="grouped_bar_chart" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("grouped_bar_chart")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"x":["giraffes","orangutans","monkeys"],"y":[20,14,23],"type":"bar","name":"SF Zoo"};
var trace_1 = {"x":["giraffes","orangutans","monkeys"],"y":[12,18,29],"type":"bar","name":"LA Zoo"};
var data = [trace_0,trace_1];
var layout = {"barmode":"group"};
        Plotly.newPlot('grouped_bar_chart', data, layout, {"responsive": true});
    };
</script>


## Stacked Bar Chart
```rust 
fn stacked_bar_chart(show: bool) {
    let animals1 = vec!["giraffes", "orangutans", "monkeys"];
    let trace1 = Bar::new(animals1, vec![20, 14, 23]).name("SF Zoo");

    let animals2 = vec!["giraffes", "orangutans", "monkeys"];
    let trace2 = Bar::new(animals2, vec![12, 18, 29]).name("LA Zoo");

    let layout = Layout::new().bar_mode(BarMode::Stack);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("stacked_bar_chart")));
}
```
<div id="stacked_bar_chart" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("stacked_bar_chart")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"x":["giraffes","orangutans","monkeys"],"y":[20,14,23],"type":"bar","name":"SF Zoo"};
var trace_1 = {"x":["giraffes","orangutans","monkeys"],"y":[12,18,29],"type":"bar","name":"LA Zoo"};
var data = [trace_0,trace_1];
var layout = {"barmode":"stack"};
        Plotly.newPlot('stacked_bar_chart', data, layout, {"responsive": true});
    };
</script>