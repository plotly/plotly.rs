# `ndarray` Support

To enable [ndarray](https://github.com/rust-ndarray/ndarray) support in [Plotly.rs](https://github.com/plotly/plotly.rs) add the following feature to your `Cargo.toml` file:
```toml
[dependencies]
plotly = { version = ">=0.7.0", features = ["plotly_ndarray"] }
```

This extends the [Plotly.rs](https://github.com/plotly/plotly.rs) API in two ways:
* `Scatter` traces can now be created using the `Scatter::from_ndarray` constructor,
* and also multiple traces can be created with the `Scatter::to_traces` method.

The full source code for the examples below can be found [here](https://github.com/plotly/plotly.rs/tree/main/examples/ndarray_support).

## `ndarray` Traces

The following imports have been used to produce the plots below:

```rust
use plotly::common::{Mode};
use plotly::{Plot, Scatter};
use ndarray::{Array, Ix1, Ix2};
use plotly::ndarray::ArrayTraces;
```


### Single Trace
```rust
fn single_ndarray_trace(show: bool) {
    let n: usize = 11;
    let t: Array<f64, Ix1> = Array::range(0., 10., 10. / n as f64);
    let ys: Array<f64, Ix1> = t.iter().map(|v| (*v).powf(2.)).collect();

    let trace = Scatter::from_array(t, ys).mode(Mode::LinesMarkers);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("single_ndarray_trace")));
}
```
<div id="single_ndarray_trace" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("single_ndarray_trace")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[0.0,0.8264462809917354,3.305785123966942,7.438016528925619,13.223140495867767,20.661157024793383,29.752066115702476,40.49586776859504,52.89256198347107,66.94214876033058,82.64462809917353]};
var data = [trace_0];
var layout = {};
        Plotly.newPlot('single_ndarray_trace', data, layout, {"responsive": true});
    };
</script>

### Multiple Traces
To display a `2D` array (`Array<_, Ix2>`) you can use the `Scatter::to_traces` method. The first argument of the method represents the common axis for the traces (`x` axis) whilst the second argument contains a collection of traces. At this point it should be noted that there is some ambiguity when passing a `2D` array; namely are the traces arranged along the columns or the rows of the matrix? This ambiguity is resolved by the third argument of the `Scatter::to_traces` method. If that argument is set to `ArrayTraces::OverColumns` then the library assumes that every column represents an individual trace, alternatively if this is set to `ArrayTraces::OverRows` the assumption is that every row represents a trace.

To illustrate this distinction consider the following examples:
```rust
fn multiple_ndarray_traces_over_columns(show: bool) {
    let n: usize = 11;
    let t: Array<f64, Ix1> = Array::range(0., 10., 10. / n as f64);
    let mut ys: Array<f64, Ix2> = Array::zeros((11, 11));
    let mut count = 0.;
    for mut row in ys.columns_mut() {
        for index in 0..row.len() {
            row[index] = count + (index as f64).powf(2.);
        }
        count += 1.;
    }

    let traces =
        Scatter::default()
            .mode(Mode::LinesMarkers)
            .to_traces(t, ys, ArrayTraces::OverColumns);

    let mut plot = Plot::new();
    plot.add_traces(traces);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("multiple_ndarray_traces_over_columns")));
}
```
<div id="multiple_ndarray_traces_over_columns" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("multiple_ndarray_traces_over_columns")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[0.0,1.0,4.0,9.0,16.0,25.0,36.0,49.0,64.0,81.0,100.0]};
var trace_1 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[1.0,2.0,5.0,10.0,17.0,26.0,37.0,50.0,65.0,82.0,101.0]};
var trace_2 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[2.0,3.0,6.0,11.0,18.0,27.0,38.0,51.0,66.0,83.0,102.0]};
var trace_3 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[3.0,4.0,7.0,12.0,19.0,28.0,39.0,52.0,67.0,84.0,103.0]};
var trace_4 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[4.0,5.0,8.0,13.0,20.0,29.0,40.0,53.0,68.0,85.0,104.0]};
var trace_5 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[5.0,6.0,9.0,14.0,21.0,30.0,41.0,54.0,69.0,86.0,105.0]};
var trace_6 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[6.0,7.0,10.0,15.0,22.0,31.0,42.0,55.0,70.0,87.0,106.0]};
var trace_7 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[7.0,8.0,11.0,16.0,23.0,32.0,43.0,56.0,71.0,88.0,107.0]};
var trace_8 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[8.0,9.0,12.0,17.0,24.0,33.0,44.0,57.0,72.0,89.0,108.0]};
var trace_9 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[9.0,10.0,13.0,18.0,25.0,34.0,45.0,58.0,73.0,90.0,109.0]};
var trace_10 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[10.0,11.0,14.0,19.0,26.0,35.0,46.0,59.0,74.0,91.0,110.0]};
var data = [trace_0,trace_1,trace_2,trace_3,trace_4,trace_5,trace_6,trace_7,trace_8,trace_9,trace_10];
var layout = {};
        Plotly.newPlot('multiple_ndarray_traces_over_columns', data, layout, {"responsive": true});
    };
</script>

Replacing `ArrayTraces::OverColumns` with `ArrayTraces::OverRows` results in the following:

<div id="multiple_ndarray_traces_over_rows" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("multiple_ndarray_traces_over_rows")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0]};
var trace_1 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0]};
var trace_2 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0]};
var trace_3 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0]};
var trace_4 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[16.0,17.0,18.0,19.0,20.0,21.0,22.0,23.0,24.0,25.0,26.0]};
var trace_5 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[25.0,26.0,27.0,28.0,29.0,30.0,31.0,32.0,33.0,34.0,35.0]};
var trace_6 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[36.0,37.0,38.0,39.0,40.0,41.0,42.0,43.0,44.0,45.0,46.0]};
var trace_7 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[49.0,50.0,51.0,52.0,53.0,54.0,55.0,56.0,57.0,58.0,59.0]};
var trace_8 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[64.0,65.0,66.0,67.0,68.0,69.0,70.0,71.0,72.0,73.0,74.0]};
var trace_9 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[81.0,82.0,83.0,84.0,85.0,86.0,87.0,88.0,89.0,90.0,91.0]};
var trace_10 = {"type":"scatter","mode":"lines+markers","x":[0.0,0.9090909090909091,1.8181818181818181,2.727272727272727,3.6363636363636362,4.545454545454545,5.454545454545454,6.363636363636363,7.2727272727272725,8.181818181818182,9.09090909090909],"y":[100.0,101.0,102.0,103.0,104.0,105.0,106.0,107.0,108.0,109.0,110.0]};
var data = [trace_0,trace_1,trace_2,trace_3,trace_4,trace_5,trace_6,trace_7,trace_8,trace_9,trace_10];
var layout = {};
        Plotly.newPlot('multiple_ndarray_traces_over_rows', data, layout, {"responsive": true});
    };
</script>