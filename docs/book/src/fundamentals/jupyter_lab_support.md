# Jupyter Lab Support

As of version `0.6.0`, [Plotly.rs](https://github.com/igiagkiozis/plotly) has native support for the [EvCxR Jupyter Kernel](https://github.com/google/evcxr/tree/master/evcxr_jupyter). 

Once you've installed the required packages you'll be able to run all the examples shown here as well as all [the recipes](../recipes.md) in Jupyter Lab!


## Installation

```shell script
conda install -c plotly=4.9.0
conda install jupyterlab "ipywidgets=7.5"
```

Although there are alternative methods to enable support for the [EvCxR Jupyter Kernel](https://github.com/google/evcxr/tree/master/evcxr_jupyter), we have elected to keep the requirements consistent with what those of other languages, e.g. Julia, Python and R. This way users know what to expect; and also the folks at [Plotly](https://plotly.com/python/getting-started/#jupyter-notebook-support) have done already most of the heavy lifting to create an extension for Jupyter Lab that works very well.

Run the following to install the Plotly Jupyter Lab extension: 
```shell script
jupyter labextension install jupyterlab-plotly@4.9.0
``` 

Once this step is complete to make sure the installation so far was successful, run the following command: 
```shell script
jupyter lab
```

Open a `Python 3` kernel copy/paste the following code in a cell and run it: 
```python
import plotly.graph_objects as go
fig = go.Figure(data=go.Bar(x=['a', 'b', 'c'], y=[11, 22, 33]))
fig.show()
```
You should see the following figure:
<div id="jupyter_lab_demo_bar_chart_python" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("jupyter_lab_demo_bar_chart_python")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"x":["a","b","c"],"y":[11,22,33],"type":"bar"};
var data = [trace_0];
var layout = {};
        Plotly.newPlot('jupyter_lab_demo_bar_chart_python', data, layout, {"responsive": true});
    };
</script>

Next you need to install the [EvCxR Jupyter Kernel](https://github.com/google/evcxr/tree/master/evcxr_jupyter). Note that EvCxR requires [CMake](https://cmake.org/download/) as it has to compile ZMQ. If [CMake](https://cmake.org/download/) is already installed on your system and is in your path (to test that simply run ```cmake --version``` if that returns a version you're good to go) then continue to the next steps.

In a command line execute the following commands:
```shell script
cargo install evcxr_jupyter
evcxr_jupyter --install
```

If you're not familiar with the EvCxR kernel it would be good that you at least glance over the [EvCxR Jupyter Tour](https://github.com/google/evcxr/blob/master/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb).

## Usage

Launch Jupyter Lab: 
```shell script
jupyter lab
```

create a new notebook and select the `Rust` kernel. Then create the following three cells and execute them in order:

```shell script
:dep plotly = { version = ">=0.6.0" }
:dep itertools-num = "0.1.3"
```

```rust
extern crate plotly;
extern crate rand_distr;
extern crate itertools_num;
extern crate itertools;
```

```rust
use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::{Bar, NamedColor, Plot, Rgb, Rgba, Scatter};
use rand_distr::{Distribution, Normal, Uniform};
```

Now we're ready to start plotting!

```rust
let n: usize = 100;
let t: Vec<f64> = linspace(0., 10., n).collect();
let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();

let trace = Scatter::new(t, y).mode(Mode::Markers);
let mut plot = Plot::new();
plot.add_trace(trace);
let layout = Layout::new().height(800);
plot.set_layout(layout);
// Alternatively you can directly invoke the display method: plot.evcxr_display();
plot
```
Notice that at the last line there is no semicolon. There are two ways to display a plot in the `EvCxR` kernel, either have the plot object as shown above in the last line or directly invoke the `Plot::evcxr_display()` method on it; both have the same result. You can also find an example notebook [here](https://github.com/igiagkiozis/plotly/blob/master/plotly/examples/jupyterlab_examples.ipynb) that will periodically be updated with examples.
