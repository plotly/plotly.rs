# Box Plots

The following imports have been used to produce the plots below:

```rust
use itertools_num::linspace;
use plotly::box_plot::{BoxMean, BoxPoints};
use plotly::common::{ErrorData, ErrorType, Line, Marker, Mode, Orientation, Title};
use plotly::histogram::{Bins, Cumulative, HistFunc, HistNorm};
use plotly::layout::{Axis, BarMode, BoxMode, Layout, Margin};
use plotly::{Bar, BoxPlot, Histogram, color::{NamedColor, Rgb, Rgba}, Scatter};
use rand_distr::{Distribution, Normal, Uniform};

```

The `to_inline_html` method is used to produce the html plot displayed in this page.


## Basic Box Plot
```rust
fn basic_box_plot(show: bool) {
    let mut rng = rand::thread_rng();
    let uniform1 = Uniform::new(0.0, 1.0);
    let uniform2 = Uniform::new(1.0, 2.0);
    let n = 50;

    let mut y0 = Vec::with_capacity(n);
    let mut y1 = Vec::with_capacity(n);

    for _ in 0..n {
        y0.push(uniform1.sample(&mut rng));
        y1.push(uniform2.sample(&mut rng));
    }

    let trace1 = BoxPlot::<f64, f64>::new(y0);
    let trace2 = BoxPlot::<f64, f64>::new(y1);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("basic_box_plot"))
    );
}
```
<div id="basic_box_plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("basic_box_plot")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"box","y":[0.7441478716488275,0.29492977275975996,0.4661219145945985,0.7467745837130171,0.048981397036707675,0.6968833236522831,0.23603851583841862,0.8828492816581941,0.4275715709419532,0.826398133787096,0.3282497245688467,0.7801777043226172,0.6012063459926584,0.37920517970996026,0.5913412321671672,0.7821716575348987,0.759110819157113,0.2015123763655462,0.7070085984103462,0.1375916565735884,0.6874603871391909,0.23218384956472393,0.08335409861042553,0.17192708006127888,0.5421042498135138,0.20775053729576443,0.49439971622834955,0.9697569611658263,0.5286742361845109,0.726262119876002,0.7134314227947989,0.6391494487540905,0.7073177289633803,0.3134388763369511,0.6708494351284855,0.274662838247671,0.33846880142456026,0.12110210121537857,0.23523299409558973,0.28136662907779675,0.42770230415161414,0.8899546895988206,0.060835962260958665,0.6735681177445643,0.018346315116238188,0.7524678344212699,0.27045398306301016,0.6171004283217398,0.26744182757421164,0.8678205229567235]};
var trace_1 = {"type":"box","y":[1.2721670212396667,1.1217517528058318,1.563229138987817,1.4699019059642213,1.3321623879720104,1.6825329904541326,1.2916982407639115,1.9965787515227265,1.1988073582763934,1.4878224754852114,1.1499841877655193,1.649420370804405,1.02650078649653,1.9900391713960166,1.1587963507021748,1.5931058656417048,1.3654127337027322,1.239395884164585,1.5781517576154735,1.8572881872572888,1.6395022739221277,1.2971957357393413,1.2429560408264473,1.5360205747905589,1.1635259416565347,1.4948990774938895,1.9391927298762046,1.0941812150629873,1.3999065267223414,1.8937317581189475,1.8026796767118602,1.3255622293347185,1.9188658460283456,1.406736275250495,1.225078499818978,1.552271752783139,1.0717862560536247,1.76737618176689,1.3300762834432949,1.485315034289464,1.698660403613416,1.8643748321993958,1.7330375575684676,1.5710003984966103,1.9531516449008532,1.8313600538379835,1.8966381805201866,1.0026964137630205,1.1896402604230165,1.3004257228484553]};
var data = [trace_0,trace_1];
var layout = {};
        Plotly.newPlot('basic_box_plot', data, layout, {"responsive": true});
    };
</script>


## Box Plot that Displays the Underlying Data
```rust
fn box_plot_that_displays_the_underlying_data(show: bool) {
    let trace1 = BoxPlot::new(vec![0, 1, 1, 2, 3, 5, 8, 13, 21])
        .box_points(BoxPoints::All)
        .jitter(0.3)
        .point_pos(-1.8);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("box_plot_that_displays_the_underlying_data"))
    );
}

```
<div id="box_plot_that_displays_the_underlying_data" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("box_plot_that_displays_the_underlying_data")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"box","y":[0,1,1,2,3,5,8,13,21],"boxpoints":"all","pointpos":-1.8,"jitter":0.3};
var data = [trace_0];
var layout = {};
        Plotly.newPlot('box_plot_that_displays_the_underlying_data', data, layout, {"responsive": true});
    };
</script>


## Horizontal Box Plot
```rust
fn horizontal_box_plot(show: bool) {
    let trace1 = BoxPlot::new(vec![1, 2, 3, 4, 4, 4, 8, 9, 10]).name("Set 1");
    let trace2 = BoxPlot::new(vec![2, 3, 3, 3, 3, 5, 6, 6, 7]).name("Set 2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("horizontal_box_plot"))
    );
}
```
<div id="horizontal_box_plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("horizontal_box_plot")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"box","y":[1,2,3,4,4,4,8,9,10],"name":"Set 1"};
var trace_1 = {"type":"box","y":[2,3,3,3,3,5,6,6,7],"name":"Set 2"};
var data = [trace_0,trace_1];
var layout = {};
        Plotly.newPlot('horizontal_box_plot', data, layout, {"responsive": true});
    };
</script>


## Grouped Box Plot
```rust
fn grouped_box_plot(show: bool) {
    let x = vec![
        "day 1", "day 1", "day 1", "day 1", "day 1", "day 1", "day 2", "day 2", "day 2", "day 2",
        "day 2", "day 2",
    ];

    let trace1 = BoxPlot::new_xy(
        x.clone(),
        vec![0.2, 0.2, 0.6, 1.0, 0.5, 0.4, 0.2, 0.7, 0.9, 0.1, 0.5, 0.3],
    );
    let trace2 = BoxPlot::new_xy(
        x.clone(),
        vec![0.6, 0.7, 0.3, 0.6, 0.0, 0.5, 0.7, 0.9, 0.5, 0.8, 0.7, 0.2],
    );
    let trace3 = BoxPlot::new_xy(
        x.clone(),
        vec![0.1, 0.3, 0.1, 0.9, 0.6, 0.6, 0.9, 1.0, 0.3, 0.6, 0.8, 0.5],
    );

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    let layout = Layout::new()
        .y_axis(
            Axis::new()
                .title(Title::with_text("normalized moisture"))
                .zero_line(false),
        )
        .box_mode(BoxMode::Group);

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("grouped_box_plot"))
    );
}
```
<div id="grouped_box_plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("grouped_box_plot")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"box","x":["day 1","day 1","day 1","day 1","day 1","day 1","day 2","day 2","day 2","day 2","day 2","day 2"],"y":[0.2,0.2,0.6,1.0,0.5,0.4,0.2,0.7,0.9,0.1,0.5,0.3]};
var trace_1 = {"type":"box","x":["day 1","day 1","day 1","day 1","day 1","day 1","day 2","day 2","day 2","day 2","day 2","day 2"],"y":[0.6,0.7,0.3,0.6,0.0,0.5,0.7,0.9,0.5,0.8,0.7,0.2]};
var trace_2 = {"type":"box","x":["day 1","day 1","day 1","day 1","day 1","day 1","day 2","day 2","day 2","day 2","day 2","day 2"],"y":[0.1,0.3,0.1,0.9,0.6,0.6,0.9,1.0,0.3,0.6,0.8,0.5]};
var data = [trace_0,trace_1,trace_2];
var layout = {"yaxis":{"title":{"text":"normalized moisture"},"zeroline":false},"boxmode":"group"};
        Plotly.newPlot('grouped_box_plot', data, layout, {"responsive": true});
    };
</script>


## Box Plot Styling Outliers
```rust
fn box_plot_styling_outliers(show: bool) {
    let y = vec![
        0.75, 5.25, 5.5, 6.0, 6.2, 6.6, 6.80, 7.0, 7.2, 7.5, 7.5, 7.75, 8.15, 8.15, 8.65, 8.93,
        9.2, 9.5, 10.0, 10.25, 11.5, 12.0, 16.0, 20.90, 22.3, 23.25,
    ];
    let trace1 = BoxPlot::new(y.clone())
        .name("All Points")
        .jitter(0.3)
        .point_pos(-1.8)
        .marker(Marker::new().color(Rgb::new(7, 40, 89)))
        .box_points(BoxPoints::All);
    let trace2 = BoxPlot::new(y.clone())
        .name("Only Whiskers")
        .marker(Marker::new().color(Rgb::new(9, 56, 125)))
        .box_points(BoxPoints::False);
    let trace3 = BoxPlot::new(y.clone())
        .name("Suspected Outlier")
        .marker(
            Marker::new()
                .color(Rgb::new(8, 81, 156))
                .outlier_color(Rgba::new(219, 64, 82, 0.6))
                .line(
                    Line::new()
                        .outlier_color(Rgba::new(219, 64, 82, 1.0))
                        .outlier_width(2),
                ),
        )
        .box_points(BoxPoints::SuspectedOutliers);
    let trace4 = BoxPlot::new(y.clone())
        .name("Whiskers and Outliers")
        .marker(Marker::new().color(Rgb::new(107, 174, 214)))
        .box_points(BoxPoints::Outliers);

    let layout = Layout::new().title(Title::with_text("Box Plot Styling Outliers"));

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("box_plot_styling_outliers"))
    );
}
```
<div id="box_plot_styling_outliers" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("box_plot_styling_outliers")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"box","y":[0.75,5.25,5.5,6.0,6.2,6.6,6.8,7.0,7.2,7.5,7.5,7.75,8.15,8.15,8.65,8.93,9.2,9.5,10.0,10.25,11.5,12.0,16.0,20.9,22.3,23.25],"name":"All Points","marker":{"color":"rgb(7, 40, 89)"},"boxpoints":"all","pointpos":-1.8,"jitter":0.3};
var trace_1 = {"type":"box","y":[0.75,5.25,5.5,6.0,6.2,6.6,6.8,7.0,7.2,7.5,7.5,7.75,8.15,8.15,8.65,8.93,9.2,9.5,10.0,10.25,11.5,12.0,16.0,20.9,22.3,23.25],"name":"Only Whiskers","marker":{"color":"rgb(9, 56, 125)"},"boxpoints":false};
var trace_2 = {"type":"box","y":[0.75,5.25,5.5,6.0,6.2,6.6,6.8,7.0,7.2,7.5,7.5,7.75,8.15,8.15,8.65,8.93,9.2,9.5,10.0,10.25,11.5,12.0,16.0,20.9,22.3,23.25],"name":"Suspected Outlier","marker":{"line":{"outliercolor":"rgba(219, 64, 82, 1)","outlierwidth":2},"color":"rgb(8, 81, 156)","outliercolor":"rgba(219, 64, 82, 0.6)"},"boxpoints":"suspectedoutliers"};
var trace_3 = {"type":"box","y":[0.75,5.25,5.5,6.0,6.2,6.6,6.8,7.0,7.2,7.5,7.5,7.75,8.15,8.15,8.65,8.93,9.2,9.5,10.0,10.25,11.5,12.0,16.0,20.9,22.3,23.25],"name":"Whiskers and Outliers","marker":{"color":"rgb(107, 174, 214)"},"boxpoints":"outliers"};
var data = [trace_0,trace_1,trace_2,trace_3];
var layout = {"title":{"text":"Box Plot Styling Outliers"}};
        Plotly.newPlot('box_plot_styling_outliers', data, layout, {"responsive": true});
    };
</script>


## Box Plot Styling Mean and Standard Deviation
```rust
fn box_plot_styling_mean_and_standard_deviation(show: bool) {
    let y = vec![
        2.37, 2.16, 4.82, 1.73, 1.04, 0.23, 1.32, 2.91, 0.11, 4.51, 0.51, 3.75, 1.35, 2.98, 4.50,
        0.18, 4.66, 1.30, 2.06, 1.19,
    ];

    let trace1 = BoxPlot::new(y.clone())
        .name("Only Mean")
        .marker(Marker::new().color(Rgb::new(8, 81, 156)))
        .box_mean(BoxMean::True);
    let trace2 = BoxPlot::new(y.clone())
        .name("Mean and Standard Deviation")
        .marker(Marker::new().color(Rgb::new(8, 81, 156)))
        .box_mean(BoxMean::StandardDeviation);
    let layout = Layout::new().title(Title::with_text("Box Plot Styling Mean and Standard Deviation"));

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("box_plot_styling_mean_and_standard_deviation"))
    );
}
```
<div id="box_plot_styling_mean_and_standard_deviation" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("box_plot_styling_mean_and_standard_deviation")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"box","y":[2.37,2.16,4.82,1.73,1.04,0.23,1.32,2.91,0.11,4.51,0.51,3.75,1.35,2.98,4.5,0.18,4.66,1.3,2.06,1.19],"name":"Only Mean","marker":{"color":"rgb(8, 81, 156)"},"boxmean":true};
var trace_1 = {"type":"box","y":[2.37,2.16,4.82,1.73,1.04,0.23,1.32,2.91,0.11,4.51,0.51,3.75,1.35,2.98,4.5,0.18,4.66,1.3,2.06,1.19],"name":"Mean and Standard Deviation","marker":{"color":"rgb(8, 81, 156)"},"boxmean":"sd"};
var data = [trace_0,trace_1];
var layout = {"title":{"text":"Box Plot Styling Mean and Standard Deviation"}};
        Plotly.newPlot('box_plot_styling_mean_and_standard_deviation', data, layout, {"responsive": true});
    };
</script>


## Grouped Horizontal Box Plot
```rust
fn grouped_horizontal_box_plot(show: bool) {
    let x = vec![
        "day 1", "day 1", "day 1", "day 1", "day 1", "day 1", "day 2", "day 2", "day 2", "day 2",
        "day 2", "day 2",
    ];

    let trace1 = BoxPlot::new_xy(
        vec![0.2, 0.2, 0.6, 1.0, 0.5, 0.4, 0.2, 0.7, 0.9, 0.1, 0.5, 0.3],
        x.clone(),
    )
        .name("Kale")
        .marker(Marker::new().color("3D9970"))
        .box_mean(BoxMean::False)
        .orientation(Orientation::Horizontal);
    let trace2 = BoxPlot::new_xy(
        vec![0.6, 0.7, 0.3, 0.6, 0.0, 0.5, 0.7, 0.9, 0.5, 0.8, 0.7, 0.2],
        x.clone(),
    )
        .name("Radishes")
        .marker(Marker::new().color("FF4136"))
        .box_mean(BoxMean::False)
        .orientation(Orientation::Horizontal);
    let trace3 = BoxPlot::new_xy(
        vec![0.1, 0.3, 0.1, 0.9, 0.6, 0.6, 0.9, 1.0, 0.3, 0.6, 0.8, 0.5],
        x.clone(),
    )
        .name("Carrots")
        .marker(Marker::new().color("FF851B"))
        .box_mean(BoxMean::False)
        .orientation(Orientation::Horizontal);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    let layout = Layout::new()
        .title(Title::with_text("Grouped Horizontal Box Plot"))
        .x_axis(
            Axis::new()
                .title(Title::with_text("normalized moisture"))
                .zero_line(false),
        )
        .box_mode(BoxMode::Group);

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("grouped_horizontal_box_plot"))
    );
}
```
<div id="grouped_horizontal_box_plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("grouped_horizontal_box_plot")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"box","x":[0.2,0.2,0.6,1.0,0.5,0.4,0.2,0.7,0.9,0.1,0.5,0.3],"y":["day 1","day 1","day 1","day 1","day 1","day 1","day 2","day 2","day 2","day 2","day 2","day 2"],"name":"Kale","orientation":"h","marker":{"color":"#3D9970"},"boxmean":false};
var trace_1 = {"type":"box","x":[0.6,0.7,0.3,0.6,0.0,0.5,0.7,0.9,0.5,0.8,0.7,0.2],"y":["day 1","day 1","day 1","day 1","day 1","day 1","day 2","day 2","day 2","day 2","day 2","day 2"],"name":"Radishes","orientation":"h","marker":{"color":"#FF4136"},"boxmean":false};
var trace_2 = {"type":"box","x":[0.1,0.3,0.1,0.9,0.6,0.6,0.9,1.0,0.3,0.6,0.8,0.5],"y":["day 1","day 1","day 1","day 1","day 1","day 1","day 2","day 2","day 2","day 2","day 2","day 2"],"name":"Carrots","orientation":"h","marker":{"color":"#FF851B"},"boxmean":false};
var data = [trace_0,trace_1,trace_2];
var layout = {"title":{"text":"Grouped Horizontal Box Plot"},"xaxis":{"title":{"text":"normalized moisture"},"zeroline":false},"boxmode":"group"};
        Plotly.newPlot('grouped_horizontal_box_plot', data, layout, {"responsive": true});
    };
</script>



## Fully Styled Box Plot
```rust
fn fully_styled_box_plot(show: bool) {
    let rnd_sample = |num, mul| -> Vec<f64> {
        let mut v: Vec<f64> = Vec::with_capacity(num);
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new(0.0, mul);
        for _ in 0..num {
            v.push(uniform.sample(&mut rng));
        }
        v
    };

    let x_data = vec![
        "Carmelo<br>Anthony",
        "Dwyane<br>Wade",
        "Deron<br>Williams",
        "Brook<br>Lopez",
        "Damian<br>Lillard",
        "David<br>West",
        "Blake<br>Griffin",
        "David<br>Lee",
        "Demar<br>Derozan",
    ];
    let y_data = vec![
        rnd_sample(30, 10.0),
        rnd_sample(30, 20.0),
        rnd_sample(30, 25.0),
        rnd_sample(30, 40.0),
        rnd_sample(30, 45.0),
        rnd_sample(30, 30.0),
        rnd_sample(30, 20.0),
        rnd_sample(30, 15.0),
        rnd_sample(30, 43.0),
    ];

    let mut plot = Plot::new();
    let layout = Layout::new()
        .title(Title::new(
            "Points Scored by the Top 9 Scoring NBA Players in 2012",
        ))
        .y_axis(
            Axis::new()
                .auto_range(true)
                .show_grid(true)
                .zero_line(true)
                .dtick(5.0)
                .grid_color(Rgb::new(255, 255, 255))
                .grid_width(1)
                .zero_line_color(Rgb::new(255, 255, 255))
                .zero_line_width(2),
        )
        .margin(Margin::new().left(40).right(30).bottom(80).top(100))
        .paper_background_color(Rgb::new(243, 243, 243))
        .plot_background_color(Rgb::new(243, 243, 243))
        .show_legend(false);
    plot.set_layout(layout);

    for index in 0..x_data.len() {
        let trace = BoxPlot::new(y_data[index].clone())
            .name(x_data[index])
            .box_points(BoxPoints::All)
            .jitter(0.5)
            .whisker_width(0.2)
            .marker(Marker::new().size(6))
            .line(Line::new().width(2.0));
        plot.add_trace(trace);
    }
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("fully_styled_box_plot"))
    );
}
```
<div id="fully_styled_box_plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("fully_styled_box_plot")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"box","y":[0.5935143400706155,4.615688052510336,4.610643779218757,4.326899697465021,5.791996440211489,5.7645813136221875,2.379309740455764,2.518892512042681,2.7170005554314858,6.749332668713226,9.887516912471852,7.265331505568813,9.276241954999879,2.7409115033588693,5.4827893935124905,6.730487208664757,7.315803838748041,6.635262284036139,8.510029791117919,1.5131595018808075,6.683931408320141,1.8627737223780794,8.581922639916007,6.103546367495152,8.037601252932179,3.2139064642272586,2.5283094581643595,7.334108118788832,4.154436990397037,0.7090342192515364],"name":"Carmelo<br>Anthony","marker":{"size":6},"line":{"width":2.0},"boxpoints":"all","whiskerwidth":0.2,"jitter":0.5};
var trace_1 = {"type":"box","y":[1.7682156360154444,1.3481642756056234,11.067450601796715,0.8390212895136706,17.81007577342902,8.867275131701655,5.1861707620898745,4.982030799963413,2.502066335634816,1.86704008685044,5.393878256082161,10.289689041577521,16.012961733409718,12.214584953641324,3.036630115450083,6.109565928573368,18.7686600511643,19.84627904911396,10.628152565923038,6.707715211777372,9.157214793414052,6.826525769227247,3.777044806485348,11.515508862157503,3.157129123204876,2.077107119369943,4.231345751088442,18.411202963919777,2.2695041615058464,10.194081796063292],"name":"Dwyane<br>Wade","marker":{"size":6},"line":{"width":2.0},"boxpoints":"all","whiskerwidth":0.2,"jitter":0.5};
var trace_2 = {"type":"box","y":[4.600159187088932,7.82640897029741,20.91787951911505,6.680225007889789,12.47920658322133,21.933550300433886,11.736786305701042,3.256540226369059,11.696711080110344,21.059349248641567,14.09919721434642,11.506666393283165,6.984433017814117,17.85761910585255,23.93520607297254,13.192223971374617,2.98774820111003,16.7138566510071,12.221874346581046,0.9756262175858066,3.736839522932478,21.832009943612668,23.858572747668795,15.745337711689944,14.149699255956483,3.32504560866399,3.7312420811217937,14.496257693175368,9.189445140771053,8.772654912876288],"name":"Deron<br>Williams","marker":{"size":6},"line":{"width":2.0},"boxpoints":"all","whiskerwidth":0.2,"jitter":0.5};
var trace_3 = {"type":"box","y":[25.998096585489048,15.126963647451026,36.84773298080245,6.613610374884296,12.819263495259001,18.882035591182813,14.294434159702911,27.81766344021814,29.803513119107876,24.726558362491133,8.590495578955384,8.956193685845477,21.883061258304288,12.618539958214008,35.766071234230246,11.820895107671285,17.195394709457766,32.591054065510775,6.741345112143184,28.792183317125648,4.750484341970367,0.1888076452979348,10.622387293384792,32.978368976316396,3.2845403571739062,13.020157777990597,17.81819131791309,33.9575857029105,8.055749655771463,5.234547997603309],"name":"Brook<br>Lopez","marker":{"size":6},"line":{"width":2.0},"boxpoints":"all","whiskerwidth":0.2,"jitter":0.5};
var trace_4 = {"type":"box","y":[37.86419109236146,28.67334964115528,14.278114019388093,32.856342722164825,3.6029435378641663,31.939963221331823,19.693458207544225,16.05278751279672,18.19005431039753,5.037437986931218,0.5811670501770905,28.260325499053017,24.959108894271694,37.807176168785276,17.47176192688627,34.21310692948772,10.680347278365577,11.45811245639474,44.34341599891414,5.704432902172767,38.02974359581067,20.669503032822483,7.279739943106072,29.667346060826915,39.034962407141734,38.93594749110978,16.50055507842856,29.85448361991831,32.50520500615463,25.96428912413287],"name":"Damian<br>Lillard","marker":{"size":6},"line":{"width":2.0},"boxpoints":"all","whiskerwidth":0.2,"jitter":0.5};
var trace_5 = {"type":"box","y":[23.52473273079021,15.535846175968867,21.642256106918655,9.805534747723442,11.45423945447009,4.1774563174907,24.14173423554371,8.816926269362932,3.6783276346627014,11.063089922453045,23.373177577382535,28.867111552974563,3.2028066984312287,28.245527568457554,4.108087447220711,18.026589110361765,5.087934662612518,26.281930175143614,0.18272104350828045,17.267658564680907,23.877431981335217,19.64111929133174,23.590825519348172,17.836126839933225,12.800313969514729,19.599188047188566,18.67540307168846,12.698178384583798,12.021748150465807,26.309622437764844],"name":"David<br>West","marker":{"size":6},"line":{"width":2.0},"boxpoints":"all","whiskerwidth":0.2,"jitter":0.5};
var trace_6 = {"type":"box","y":[17.363579920499205,6.313488277675843,10.127811573394485,4.458988287189167,5.344504506073746,5.025085846707871,6.203095285646305,18.287621123038065,12.287029935362712,12.358448278602738,2.2465721971568353,19.85977208959998,11.887600940009975,10.142686366102529,7.355801280233245,19.954039799940027,19.275910728719335,4.817997242054242,7.447650727661106,13.610926504320489,17.326994525575007,5.0491511255852695,13.635951845414374,10.673909207599426,4.4483952275310035,8.951918849752797,14.587171690874623,7.375207072527017,7.154812588986905,3.8426066382889124],"name":"Blake<br>Griffin","marker":{"size":6},"line":{"width":2.0},"boxpoints":"all","whiskerwidth":0.2,"jitter":0.5};
var trace_7 = {"type":"box","y":[12.696269138091786,3.1388374058942716,10.43834066900343,14.8750162969579,6.231546713506826,9.91489134208716,2.6314770277459676,1.8638235021202199,4.699377042200194,8.053452093454844,4.955769795427986,8.401639848880546,9.488148689964506,13.207029113369956,12.864568851051652,1.5437076789715765,10.96052789599658,6.7221086364779294,12.698966345959736,13.126920034768624,3.268666726526621,0.8946597857057514,2.461284421179357,6.694685907156051,11.624785830379814,12.275801086142268,1.860063571446492,5.472195387198278,2.1008185732950557,10.408437914744885],"name":"David<br>Lee","marker":{"size":6},"line":{"width":2.0},"boxpoints":"all","whiskerwidth":0.2,"jitter":0.5};
var trace_8 = {"type":"box","y":[12.822358569594897,27.96667919443002,16.341979023526257,42.93093812351294,13.880322446590853,28.79177132403798,12.368060572100305,11.92650713913392,3.811740981234818,5.949472953544888,9.81924157743332,13.375200617305605,29.73176530206266,37.1208104066369,14.320994829379973,11.577271602539271,13.217529663037078,30.37313239470977,10.136308222862933,13.036833886020448,39.88506705548082,14.583860248422493,12.945813959282374,0.5617479276588344,25.048603342464297,28.843812822647955,16.157586660271324,38.981690000677624,21.92294775405139,14.91456000947055],"name":"Demar<br>Derozan","marker":{"size":6},"line":{"width":2.0},"boxpoints":"all","whiskerwidth":0.2,"jitter":0.5};
var data = [trace_0,trace_1,trace_2,trace_3,trace_4,trace_5,trace_6,trace_7,trace_8];
var layout = {"title":{"text":"Points Scored by the Top 9 Scoring NBA Players in 2012"},"showlegend":false,"margin":{"l":40,"r":30,"t":100,"b":80},"paper_bgcolor":"rgb(243, 243, 243)","plot_bgcolor":"rgb(243, 243, 243)","yaxis":{"auto_range":true,"dtick":5.0,"showgrid":true,"gridcolor":"rgb(255, 255, 255)","gridwidth":1,"zeroline":true,"zerolinecolor":"rgb(255, 255, 255)","zerolinewidth":2}};
        Plotly.newPlot('fully_styled_box_plot', data, layout, {"responsive": true});
    };
</script>