# Subplots

The following imports have been used to produce the plots below:

```rust
use plotly::common::{Font, Side, Title};
use plotly::layout::{Axis, GridPattern, Layout, LayoutGrid, Legend, RowOrder};
use plotly::{Plot, Rgb, Scatter};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.


## Simple Subplot
```rust
fn simple_subplot(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![20, 30, 40], vec![50, 60, 70])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(1)
            .columns(2)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);
        if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("simple_subplot")).unwrap()
    );
}
```
<div id="simple_subplot" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("simple_subplot")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3],"y":[4,5,6],"name":"trace1"};
var trace_1 = {"type":"scatter","x":[20,30,40],"y":[50,60,70],"name":"trace2","xaxis":"x2","yaxis":"y2"};
var data = [trace_0,trace_1];
var layout = {"grid":{"rows":1,"columns":2,"pattern":"independent"}};
        Plotly.newPlot('simple_subplot', data, layout, {"responsive": true});
    };
</script>


## Custom Sized Subplot
```rust
fn custom_sized_subplot(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![20, 30, 40], vec![50, 60, 70])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .x_axis(Axis::new().domain(&[0., 0.7]))
        .y_axis2(Axis::new().anchor("x2"))
        .x_axis2(Axis::new().domain(&[0.8, 1.]));
    plot.set_layout(layout);
        if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("custom_sized_subplot")).unwrap()
    );
}
```
<div id="custom_sized_subplot" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("custom_sized_subplot")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3],"y":[4,5,6],"name":"trace1"};
var trace_1 = {"type":"scatter","x":[20,30,40],"y":[50,60,70],"name":"trace2","xaxis":"x2","yaxis":"y2"};
var data = [trace_0,trace_1];
var layout = {"xaxis":{"domain":[0.0,0.7]},"xaxis2":{"domain":[0.8,1.0]},"yaxis2":{"anchor":"x2"}};
        Plotly.newPlot('custom_sized_subplot', data, layout, {"responsive": true});
    };
</script>


## Multiple Subplots
```rust
fn multiple_subplots(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![20, 30, 40], vec![50, 60, 70])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");
    let trace3 = Scatter::new(vec![300, 400, 500], vec![600, 700, 800])
        .x_axis("x3")
        .y_axis("y3");
    let trace4 = Scatter::new(vec![4000, 5000, 6000], vec![7000, 8000, 9000])
        .x_axis("x4")
        .y_axis("y4");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(2)
            .columns(2)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);
        if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("multiple_subplots")).unwrap()
    );
}
```
<div id="multiple_subplots" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("multiple_subplots")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3],"y":[4,5,6],"name":"trace1"};
var trace_1 = {"type":"scatter","x":[20,30,40],"y":[50,60,70],"name":"trace2","xaxis":"x2","yaxis":"y2"};
var trace_2 = {"type":"scatter","x":[300,400,500],"y":[600,700,800],"xaxis":"x3","yaxis":"y3"};
var trace_3 = {"type":"scatter","x":[4000,5000,6000],"y":[7000,8000,9000],"xaxis":"x4","yaxis":"y4"};
var data = [trace_0,trace_1,trace_2,trace_3];
var layout = {"grid":{"rows":2,"columns":2,"pattern":"independent"}};
        Plotly.newPlot('multiple_subplots', data, layout, {"responsive": true});
    };
</script>


## Stacked Subplots
```rust
fn stacked_subplots(show: bool) {
    let trace1 = Scatter::new(vec![0, 1, 2], vec![10, 11, 12]).name("trace1");
    let trace2 = Scatter::new(vec![2, 3, 4], vec![100, 110, 120])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");
    let trace3 = Scatter::new(vec![3, 4, 5], vec![1000, 1100, 1200])
        .x_axis("x3")
        .y_axis("y3");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(3)
            .columns(1)
            .pattern(GridPattern::Independent)
            .row_order(RowOrder::BottomToTop),
    );
    plot.set_layout(layout);
        if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("stacked_subplots")).unwrap()
    );
}
```
<div id="stacked_subplots" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("stacked_subplots")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[0,1,2],"y":[10,11,12],"name":"trace1"};
var trace_1 = {"type":"scatter","x":[2,3,4],"y":[100,110,120],"name":"trace2","xaxis":"x2","yaxis":"y2"};
var trace_2 = {"type":"scatter","x":[3,4,5],"y":[1000,1100,1200],"xaxis":"x3","yaxis":"y3"};
var data = [trace_0,trace_1,trace_2];
var layout = {"grid":{"rows":3,"roworder":"bottom to top","columns":1,"pattern":"independent"}};
        Plotly.newPlot('stacked_subplots', data, layout, {"responsive": true});
    };
</script>


## Stacked Subplots with Shared X Axis
```rust
fn stacked_subplots_with_shared_x_axis(show: bool) {
    let trace1 = Scatter::new(vec![0, 1, 2], vec![10, 11, 12]).name("trace1");
    let trace2 = Scatter::new(vec![2, 3, 4], vec![100, 110, 120])
        .name("trace2")
        .y_axis("y2");
    let trace3 = Scatter::new(vec![3, 4, 5], vec![1000, 1100, 1200]).y_axis("y3");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    let layout = Layout::new()
        .y_axis(Axis::new().domain(&[0., 0.33]))
        .legend(Legend::new().trace_order("reversed"))
        .y_axis2(Axis::new().domain(&[0.33, 0.66]))
        .y_axis3(Axis::new().domain(&[0.66, 1.]));
    plot.set_layout(layout);
        if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("stacked_subplots_with_shared_x_axis")).unwrap()
    );
}
```
<div id="stacked_subplots_with_shared_x_axis" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("stacked_subplots_with_shared_x_axis")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[0,1,2],"y":[10,11,12],"name":"trace1"};
var trace_1 = {"type":"scatter","x":[2,3,4],"y":[100,110,120],"name":"trace2","yaxis":"y2"};
var trace_2 = {"type":"scatter","x":[3,4,5],"y":[1000,1100,1200],"yaxis":"y3"};
var data = [trace_0,trace_1,trace_2];
var layout = {"legend":{"traceorder":"reversed"},"yaxis":{"domain":[0.0,0.33]},"yaxis2":{"domain":[0.33,0.66]},"yaxis3":{"domain":[0.66,1.0]}};
        Plotly.newPlot('stacked_subplots_with_shared_x_axis', data, layout, {"responsive": true});
    };
</script>


## Multiple Custom Sized Subplots
```rust
fn multiple_custom_sized_subplots(show: bool) {
    let trace1 = Scatter::new(vec![1, 2], vec![1, 2]).name("(1,1)");
    let trace2 = Scatter::new(vec![1, 2], vec![1, 2])
        .name("(1,2,1)")
        .x_axis("x2")
        .y_axis("y2");
    let trace3 = Scatter::new(vec![1, 2], vec![1, 2])
        .name("(1,2,2)")
        .x_axis("x3")
        .y_axis("y3");
    let trace4 = Scatter::new(vec![1, 2], vec![1, 2])
        .name("{(2,1), (2,2)}")
        .x_axis("x4")
        .y_axis("y4");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);

    let layout = Layout::new()
        .title(Title::new("Multiple Custom Sized Subplots"))
        .x_axis(Axis::new().domain(&[0., 0.45]).anchor("y1"))
        .y_axis(Axis::new().domain(&[0.5, 1.]).anchor("x1"))
        .x_axis2(Axis::new().domain(&[0.55, 1.]).anchor("y2"))
        .y_axis2(Axis::new().domain(&[0.8, 1.]).anchor("x2"))
        .x_axis3(Axis::new().domain(&[0.55, 1.]).anchor("y3"))
        .y_axis3(Axis::new().domain(&[0.5, 0.75]).anchor("x3"))
        .x_axis4(Axis::new().domain(&[0., 1.]).anchor("y4"))
        .y_axis4(Axis::new().domain(&[0., 0.45]).anchor("x4"));
    plot.set_layout(layout);
        if show {
        plot.show().unwrap();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("multiple_custom_sized_subplots")).unwrap()
    );
}
```
<div id="multiple_custom_sized_subplots" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("multiple_custom_sized_subplots")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2],"y":[1,2],"name":"(1,1)"};
var trace_1 = {"type":"scatter","x":[1,2],"y":[1,2],"name":"(1,2,1)","xaxis":"x2","yaxis":"y2"};
var trace_2 = {"type":"scatter","x":[1,2],"y":[1,2],"name":"(1,2,2)","xaxis":"x3","yaxis":"y3"};
var trace_3 = {"type":"scatter","x":[1,2],"y":[1,2],"name":"{(2,1), (2,2)}","xaxis":"x4","yaxis":"y4"};
var data = [trace_0,trace_1,trace_2,trace_3];
var layout = {"title":{"text":"Multiple Custom Sized Subplots"},"xaxis":{"anchor":"y1","domain":[0.0,0.45]},"yaxis":{"anchor":"x1","domain":[0.5,1.0]},"xaxis2":{"anchor":"y2","domain":[0.55,1.0]},"yaxis2":{"anchor":"x2","domain":[0.8,1.0]},"xaxis3":{"anchor":"y3","domain":[0.55,1.0]},"yaxis3":{"anchor":"x3","domain":[0.5,0.75]},"xaxis4":{"anchor":"y4","domain":[0.0,1.0]},"yaxis4":{"anchor":"x4","domain":[0.0,0.45]}};
        Plotly.newPlot('multiple_custom_sized_subplots', data, layout, {"responsive": true});
    };
</script>