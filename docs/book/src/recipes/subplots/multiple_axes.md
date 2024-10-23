# Multiple Axes

The following imports have been used to produce the plots below:

```rust
use plotly::common::{Font, AxisSide, Title};
use plotly::layout::{Axis, GridPattern, Layout, LayoutGrid, Legend, RowOrder};
use plotly::{Plot, Rgb, Scatter};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Two Y Axes
```rust
fn two_y_axes(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![40, 50, 60]).name("trace1");
    let trace2 = Scatter::new(vec![2, 3, 4], vec![4, 5, 6])
        .name("trace2")
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .title(Title::with_text("Double Y Axis Example"))
        .y_axis(Axis::new().title(Title::with_text("yaxis title")))
        .y_axis2(
            Axis::new()
                .title(Title::with_text("yaxis2 title").font(Font::new().color(Rgb::new(148, 103, 189))))
                .tick_font(Font::new().color(Rgb::new(148, 103, 189)))
                .overlaying("y")
                .side(AxisSide::Right),
        );
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("two_y_axes")));
}
```
<div id="two_y_axes" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("two_y_axes")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3],"y":[40,50,60],"name":"trace1"};
var trace_1 = {"type":"scatter","x":[2,3,4],"y":[4,5,6],"name":"trace2","yaxis":"y2"};
var data = [trace_0,trace_1];
var layout = {"title":{"text":"Double Y Axis Example"},"yaxis":{"title":{"text":"yaxis title"}},"yaxis2":{"title":{"text":"yaxis2 title","font":{"color":"rgb(148, 103, 189)"}},"tickfont":{"color":"rgb(148, 103, 189)"},"side":"right","overlaying":"y"}};
        Plotly.newPlot('two_y_axes', data, layout, {"responsive": true});
    };
</script>


## Multiple Axes
```rust
fn multiple_axes(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![2, 3, 4], vec![40, 50, 60])
        .name("trace2")
        .y_axis("y2");
    let trace3 = Scatter::new(vec![4, 5, 6], vec![40_000, 50_000, 60_000]).y_axis("y3");
    let trace4 = Scatter::new(vec![5, 6, 7], vec![400_000, 500_000, 600_000]).y_axis("y4");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);

    let layout = Layout::new()
        .title(Title::with_text("multiple y-axes example"))
        .width(800)
        .x_axis(Axis::new().domain(&[0.3, 0.7]))
        .y_axis(
            Axis::new()
                .title(Title::with_text("yaxis title").font(Font::new().color("#1f77b4")))
                .tick_font(Font::new().color("#1f77b4")),
        )
        .y_axis2(
            Axis::new()
                .title(Title::with_text("yaxis2 title").font(Font::new().color("#ff7f0e")))
                .tick_font(Font::new().color("#ff7f0e"))
                .anchor("free")
                .overlaying("y")
                .side(AxisSide::Left)
                .position(0.15),
        )
        .y_axis3(
            Axis::new()
                .title(Title::with_text("yaxis3 title").font(Font::new().color("#d62728")))
                .tick_font(Font::new().color("#d62728"))
                .anchor("x")
                .overlaying("y")
                .side(AxisSide::Right),
        )
        .y_axis4(
            Axis::new()
                .title(Title::with_text("yaxis4 title").font(Font::new().color("#9467bd")))
                .tick_font(Font::new().color("#9467bd"))
                .anchor("free")
                .overlaying("y")
                .side(AxisSide::Right)
                .position(0.85),
        );
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("multiple_axes")));
}
```
<div id="multiple_axes" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("multiple_axes")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3],"y":[4,5,6],"name":"trace1"};
var trace_1 = {"type":"scatter","x":[2,3,4],"y":[40,50,60],"name":"trace2","yaxis":"y2"};
var trace_2 = {"type":"scatter","x":[4,5,6],"y":[40000,50000,60000],"yaxis":"y3"};
var trace_3 = {"type":"scatter","x":[5,6,7],"y":[400000,500000,600000],"yaxis":"y4"};
var data = [trace_0,trace_1,trace_2,trace_3];
var layout = {"title":{"text":"multiple y-axes example"},"width":800,"xaxis":{"domain":[0.3,0.7]},"yaxis":{"title":{"text":"yaxis title","font":{"color":"#1F77B4"}},"tickfont":{"color":"#1F77B4"}},"yaxis2":{"title":{"text":"yaxis2 title","font":{"color":"#FF7F0E"}},"tickfont":{"color":"#FF7F0E"},"anchor":"free","side":"left","overlaying":"y","position":0.15},"yaxis3":{"title":{"text":"yaxis3 title","font":{"color":"#D62728"}},"tickfont":{"color":"#D62728"},"anchor":"x","side":"right","overlaying":"y"},"yaxis4":{"title":{"text":"yaxis4 title","font":{"color":"#9467BD"}},"tickfont":{"color":"#9467BD"},"anchor":"free","side":"right","overlaying":"y","position":0.85}};
        Plotly.newPlot('multiple_axes', data, layout, {"responsive": true});
    };
</script>