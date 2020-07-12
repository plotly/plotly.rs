# Line Charts

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


## Adding Names to Line and Scatter Plot
```rust
fn adding_names_to_line_and_scatter_plot(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .mode(Mode::Markers)
        .name("Scatter");
    let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
        .mode(Mode::Lines)
        .name("Lines");
    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
        .mode(Mode::LinesMarkers)
        .name("Scatter + Lines");

    let layout = Layout::new().title(Title::new("Adding Names to Line and Scatter Plot"));
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("adding_names_to_line_and_scatter_plot")));
}

```
<div id="adding_names_to_line_and_scatter_plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("adding_names_to_line_and_scatter_plot")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3,4],"y":[10,15,13,17],"name":"Scatter","mode":"markers"};
var trace_1 = {"type":"scatter","x":[2,3,4,5],"y":[16,5,11,9],"name":"Lines","mode":"lines"};
var trace_2 = {"type":"scatter","x":[1,2,3,4],"y":[12,9,15,12],"name":"Scatter + Lines","mode":"lines+markers"};
var data = [trace_0,trace_1,trace_2];
var layout = {"title":{"text":"Adding Names to Line and Scatter Plot"}};
        Plotly.newPlot('adding_names_to_line_and_scatter_plot', data, layout, {"responsive": true});
    };
</script>


## Line and Scatter Styling
```rust
fn line_and_scatter_styling(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .mode(Mode::Markers)
        .name("trace1")
        .marker(Marker::new().color(Rgb::new(219, 64, 82)).size(12));
    let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
        .mode(Mode::Lines)
        .name("trace2")
        .line(Line::new().color(Rgb::new(55, 128, 191)).width(3.0));
    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
        .mode(Mode::LinesMarkers)
        .name("trace3")
        .marker(Marker::new().color(Rgb::new(128, 0, 128)).size(12))
        .line(Line::new().color(Rgb::new(128, 0, 128)).width(1.0));

    let layout = Layout::new().title(Title::new("Line and Scatter Styling"));
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("line_and_scatter_styling")));
}
```
<div id="line_and_scatter_styling" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("line_and_scatter_styling")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3,4],"y":[10,15,13,17],"name":"trace1","mode":"markers","marker":{"size":12,"color":"rgb(219, 64, 82)"}};
var trace_1 = {"type":"scatter","x":[2,3,4,5],"y":[16,5,11,9],"name":"trace2","mode":"lines","line":{"width":3.0,"color":"rgb(55, 128, 191)"}};
var trace_2 = {"type":"scatter","x":[1,2,3,4],"y":[12,9,15,12],"name":"trace3","mode":"lines+markers","marker":{"size":12,"color":"rgb(128, 0, 128)"},"line":{"width":1.0,"color":"rgb(128, 0, 128)"}};
var data = [trace_0,trace_1,trace_2];
var layout = {"title":{"text":"Line and Scatter Styling"}};
        Plotly.newPlot('line_and_scatter_styling', data, layout, {"responsive": true});
    };
</script>

## Styling Line Plot
```rust
fn styling_line_plot(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .mode(Mode::Markers)
        .name("Red")
        .line(Line::new().color(Rgb::new(219, 64, 82)).width(3.0));
    let trace2 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
        .mode(Mode::LinesMarkers)
        .name("Blue")
        .line(Line::new().color(Rgb::new(55, 128, 191)).width(1.0));

    let layout = Layout::new()
        .title(Title::new("Styling Line Plot"))
        .width(500)
        .height(500);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("styling_line_plot")));
}
```
<div id="styling_line_plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("styling_line_plot")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3,4],"y":[10,15,13,17],"name":"Red","mode":"markers","line":{"width":3.0,"color":"rgb(219, 64, 82)"}};
var trace_1 = {"type":"scatter","x":[1,2,3,4],"y":[12,9,15,12],"name":"Blue","mode":"lines+markers","line":{"width":1.0,"color":"rgb(55, 128, 191)"}};
var data = [trace_0,trace_1];
var layout = {"title":{"text":"Styling Line Plot"},"width":500,"height":500};
        Plotly.newPlot('styling_line_plot', data, layout, {"responsive": true});
    };
</script>

## Line Shape Options for Interpolation
```rust
fn line_shape_options_for_interpolation(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4, 5], vec![1, 3, 2, 3, 1])
        .mode(Mode::LinesMarkers)
        .name("linear")
        .line(Line::new().shape(LineShape::Linear));
    let trace2 = Scatter::new(vec![1, 2, 3, 4, 5], vec![6, 8, 7, 8, 6])
        .mode(Mode::LinesMarkers)
        .name("spline")
        .line(Line::new().shape(LineShape::Spline));
    let trace3 = Scatter::new(vec![1, 2, 3, 4, 5], vec![11, 13, 12, 13, 11])
        .mode(Mode::LinesMarkers)
        .name("vhv")
        .line(Line::new().shape(LineShape::Vhv));
    let trace4 = Scatter::new(vec![1, 2, 3, 4, 5], vec![16, 18, 17, 18, 16])
        .mode(Mode::LinesMarkers)
        .name("hvh")
        .line(Line::new().shape(LineShape::Hvh));
    let trace5 = Scatter::new(vec![1, 2, 3, 4, 5], vec![21, 23, 22, 23, 21])
        .mode(Mode::LinesMarkers)
        .name("vh")
        .line(Line::new().shape(LineShape::Vh));
    let trace6 = Scatter::new(vec![1, 2, 3, 4, 5], vec![26, 28, 27, 28, 26])
        .mode(Mode::LinesMarkers)
        .name("hv")
        .line(Line::new().shape(LineShape::Hv));

    let mut plot = Plot::new();
    let layout = Layout::new().legend(
        Legend::new()
            .y(0.5)
            .trace_order("reversed")
            .font(Font::new().size(16)),
    );
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);
    plot.show_png(1024, 680);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("line_shape_options_for_interpolation")));
}
```
<div id="line_shape_options_for_interpolation" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("line_shape_options_for_interpolation")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3,4,5],"y":[1,3,2,3,1],"name":"linear","mode":"lines+markers","line":{"shape":"linear"}};
var trace_1 = {"type":"scatter","x":[1,2,3,4,5],"y":[6,8,7,8,6],"name":"spline","mode":"lines+markers","line":{"shape":"spline"}};
var trace_2 = {"type":"scatter","x":[1,2,3,4,5],"y":[11,13,12,13,11],"name":"vhv","mode":"lines+markers","line":{"shape":"vhv"}};
var trace_3 = {"type":"scatter","x":[1,2,3,4,5],"y":[16,18,17,18,16],"name":"hvh","mode":"lines+markers","line":{"shape":"hvh"}};
var trace_4 = {"type":"scatter","x":[1,2,3,4,5],"y":[21,23,22,23,21],"name":"vh","mode":"lines+markers","line":{"shape":"vh"}};
var trace_5 = {"type":"scatter","x":[1,2,3,4,5],"y":[26,28,27,28,26],"name":"hv","mode":"lines+markers","line":{"shape":"hv"}};
var data = [trace_0,trace_1,trace_2,trace_3,trace_4,trace_5];
var layout = {"legend":{"font":{"size":16},"traceorder":"reversed","y":0.5}};
        Plotly.newPlot('line_shape_options_for_interpolation', data, layout, {"responsive": true});
    };
</script>

## Line Dash
```rust
fn line_dash(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4, 5], vec![1, 3, 2, 3, 1])
        .mode(Mode::LinesMarkers)
        .name("solid")
        .line(Line::new().dash(DashType::Solid));
    let trace2 = Scatter::new(vec![1, 2, 3, 4, 5], vec![6, 8, 7, 8, 6])
        .mode(Mode::LinesMarkers)
        .name("dashdot")
        .line(Line::new().dash(DashType::DashDot));
    let trace3 = Scatter::new(vec![1, 2, 3, 4, 5], vec![11, 13, 12, 13, 11])
        .mode(Mode::LinesMarkers)
        .name("dash")
        .line(Line::new().dash(DashType::Dash));
    let trace4 = Scatter::new(vec![1, 2, 3, 4, 5], vec![16, 18, 17, 18, 16])
        .mode(Mode::LinesMarkers)
        .name("dot")
        .line(Line::new().dash(DashType::Dot));
    let trace5 = Scatter::new(vec![1, 2, 3, 4, 5], vec![21, 23, 22, 23, 21])
        .mode(Mode::LinesMarkers)
        .name("longdash")
        .line(Line::new().dash(DashType::LongDash));
    let trace6 = Scatter::new(vec![1, 2, 3, 4, 5], vec![26, 28, 27, 28, 26])
        .mode(Mode::LinesMarkers)
        .name("longdashdot")
        .line(Line::new().dash(DashType::LongDashDot));

    let mut plot = Plot::new();
    let layout = Layout::new()
        .legend(
            Legend::new()
                .y(0.5)
                .trace_order("reversed")
                .font(Font::new().size(16)),
        )
        .x_axis(Axis::new().range(vec![0.95, 5.05]).auto_range(false))
        .y_axis(Axis::new().range(vec![0.0, 28.5]).auto_range(false));
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("line_dash")));
}
```
<div id="line_dash" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("line_dash")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3,4,5],"y":[1,3,2,3,1],"name":"solid","mode":"lines+markers","line":{"dash":"solid"}};
var trace_1 = {"type":"scatter","x":[1,2,3,4,5],"y":[6,8,7,8,6],"name":"dashdot","mode":"lines+markers","line":{"dash":"dashdot"}};
var trace_2 = {"type":"scatter","x":[1,2,3,4,5],"y":[11,13,12,13,11],"name":"dash","mode":"lines+markers","line":{"dash":"dash"}};
var trace_3 = {"type":"scatter","x":[1,2,3,4,5],"y":[16,18,17,18,16],"name":"dot","mode":"lines+markers","line":{"dash":"dot"}};
var trace_4 = {"type":"scatter","x":[1,2,3,4,5],"y":[21,23,22,23,21],"name":"longdash","mode":"lines+markers","line":{"dash":"longdashdot"}};
var trace_5 = {"type":"scatter","x":[1,2,3,4,5],"y":[26,28,27,28,26],"name":"longdashdot","mode":"lines+markers","line":{"dash":"longdashdot"}};
var data = [trace_0,trace_1,trace_2,trace_3,trace_4,trace_5];
var layout = {"legend":{"font":{"size":16},"traceorder":"reversed","y":0.5},"xaxis":{"auto_range":false,"range":[0.95,5.05]},"yaxis":{"auto_range":false,"range":[0.0,28.5]}};
        Plotly.newPlot('line_dash', data, layout, {"responsive": true});
    };
</script>


## Filled Lines
```rust
fn filled_lines(show: bool) {
    let x1 = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0,
        2.0, 1.0,
    ];
    let x2 = (1..=10).map(|iv| iv as f64).collect::<Vec<f64>>();
    let trace1 = Scatter::new(
        x1.clone(),
        vec![
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0,
            2.0, 1.0, 0.0,
        ],
    )
    .fill(Fill::ToZeroX)
    .fill_color(Rgba::new(0, 100, 80, 0.2))
    .line(Line::new().color(NamedColor::Transparent))
    .name("Fair")
    .show_legend(false);
    let trace2 = Scatter::new(
        x1.clone(),
        vec![
            5.5, 3.0, 5.5, 8.0, 6.0, 3.0, 8.0, 5.0, 6.0, 5.5, 4.75, 5.0, 4.0, 7.0, 2.0, 4.0, 7.0,
            4.4, 2.0, 4.5,
        ],
    )
    .fill(Fill::ToZeroX)
    .fill_color(Rgba::new(0, 176, 246, 0.2))
    .line(Line::new().color(NamedColor::Transparent))
    .name("Premium")
    .show_legend(false);
    let trace3 = Scatter::new(
        x1.clone(),
        vec![
            11.0, 9.0, 7.0, 5.0, 3.0, 1.0, 3.0, 5.0, 3.0, 1.0, -1.0, 1.0, 3.0, 1.0, -0.5, 1.0, 3.0,
            5.0, 7.0, 9.0,
        ],
    )
    .fill(Fill::ToZeroX)
    .fill_color(Rgba::new(231, 107, 243, 0.2))
    .line(Line::new().color(NamedColor::Transparent))
    .name("Fair")
    .show_legend(false);
    let trace4 = Scatter::new(
        x2.clone(),
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    )
    .line(Line::new().color(Rgb::new(0, 100, 80)))
    .name("Fair");
    let trace5 = Scatter::new(
        x2.clone(),
        vec![5.0, 2.5, 5.0, 7.5, 5.0, 2.5, 7.5, 4.5, 5.5, 5.0],
    )
    .line(Line::new().color(Rgb::new(0, 176, 246)))
    .name("Premium");
    let trace6 = Scatter::new(
        x2.clone(),
        vec![10.0, 8.0, 6.0, 4.0, 2.0, 0.0, 2.0, 4.0, 2.0, 0.0],
    )
    .line(Line::new().color(Rgb::new(231, 107, 243)))
    .name("Ideal");

    let layout = Layout::new()
        .paper_background_color(Rgb::new(255, 255, 255))
        .plot_background_color(Rgb::new(229, 229, 229))
        .x_axis(
            Axis::new()
                .grid_color(Rgb::new(255, 255, 255))
                .range(vec![1.0, 10.0])
                .show_grid(true)
                .show_line(false)
                .show_tick_labels(true)
                .tick_color(Rgb::new(127, 127, 127))
                .ticks(TicksDirection::Outside)
                .zero_line(false),
        )
        .y_axis(
            Axis::new()
                .grid_color(Rgb::new(255, 255, 255))
                .show_grid(true)
                .show_line(false)
                .show_tick_labels(true)
                .tick_color(Rgb::new(127, 127, 127))
                .ticks(TicksDirection::Outside)
                .zero_line(false),
        );

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("filled_lines")));
}
```
<div id="filled_lines" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("filled_lines")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,10.0,9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0],"y":[2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0,0.0],"name":"Fair","showlegend":false,"line":{"color":"transparent"},"fill":"tozerox","fillcolor":"rgba(0, 100, 80, 0.2)"};
var trace_1 = {"type":"scatter","x":[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,10.0,9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0],"y":[5.5,3.0,5.5,8.0,6.0,3.0,8.0,5.0,6.0,5.5,4.75,5.0,4.0,7.0,2.0,4.0,7.0,4.4,2.0,4.5],"name":"Premium","showlegend":false,"line":{"color":"transparent"},"fill":"tozerox","fillcolor":"rgba(0, 176, 246, 0.2)"};
var trace_2 = {"type":"scatter","x":[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,10.0,9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0],"y":[11.0,9.0,7.0,5.0,3.0,1.0,3.0,5.0,3.0,1.0,-1.0,1.0,3.0,1.0,-0.5,1.0,3.0,5.0,7.0,9.0],"name":"Fair","showlegend":false,"line":{"color":"transparent"},"fill":"tozerox","fillcolor":"rgba(231, 107, 243, 0.2)"};
var trace_3 = {"type":"scatter","x":[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0],"y":[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0],"name":"Fair","line":{"color":"rgb(0, 100, 80)"}};
var trace_4 = {"type":"scatter","x":[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0],"y":[5.0,2.5,5.0,7.5,5.0,2.5,7.5,4.5,5.5,5.0],"name":"Premium","line":{"color":"rgb(0, 176, 246)"}};
var trace_5 = {"type":"scatter","x":[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0],"y":[10.0,8.0,6.0,4.0,2.0,0.0,2.0,4.0,2.0,0.0],"name":"Ideal","line":{"color":"rgb(231, 107, 243)"}};
var data = [trace_0,trace_1,trace_2,trace_3,trace_4,trace_5];
var layout = {"paper_bgcolor":"rgb(255, 255, 255)","plot_bgcolor":"rgb(229, 229, 229)","xaxis":{"range":[1.0,10.0],"ticks":"outside","tickcolor":"rgb(127, 127, 127)","showticklabels":true,"showline":false,"showgrid":true,"gridcolor":"rgb(255, 255, 255)","zeroline":false},"yaxis":{"ticks":"outside","tickcolor":"rgb(127, 127, 127)","showticklabels":true,"showline":false,"showgrid":true,"gridcolor":"rgb(255, 255, 255)","zeroline":false}};
        Plotly.newPlot('filled_lines', data, layout, {"responsive": true});
    };
</script>