# Jupyter Support

As of version `0.7.0`, [Plotly.rs](https://github.com/plotly/plotly.rs) has native support for the [EvCxR Jupyter Kernel](https://github.com/evcxr/evcxr/tree/main/evcxr_jupyter).

Once you've installed the required packages you'll be able to run all the examples shown here as well as all [the recipes](../recipes.md) in Jupyter Lab!

## Installation

Install the plotly package and JupyterLab using pip or conda:

**pip**
```shell
pip install plotly jupyterlab
```

**conda**
```shell
conda install -c conda-forge plotly jupyterlab
```

No separate JupyterLab extension install is required — the plotly renderer is bundled
with the plotly package (5.x+) and JupyterLab picks it up automatically.

> **Note:** `anywidget` is required for Python's `FigureWidget` interactive features
> but is **not** needed for the Rust `lab_display()` / `notebook_display()` path.

Next, install the EvCxR Jupyter Kernel:

```shell
cargo install evcxr_jupyter
evcxr_jupyter --install
```

## Usage

Launch Jupyter Lab:

```shell
jupyter lab
```

Create a new notebook and select the `Rust` kernel. Add the plotly dependency and
display a plot:

**Cell 1**
```
:dep plotly = "0.14"
```

**Cell 2**
```rust
use plotly::{Plot, Scatter};

let trace = Scatter::new(vec![1.0, 2.0, 3.0], vec![1.0, 4.0, 9.0]);
let mut plot = Plot::new();
plot.add_trace(trace);

plot.lab_display();
```

For Jupyter Lab there are two ways to display a plot in the `EvCxR` kernel: either
have the plot object on the last line without a semicolon, or call `Plot::lab_display`
directly — both produce the same result.

For Jupyter Notebook, use `Plot::notebook_display` instead. You can find an example
notebook [here](https://github.com/plotly/plotly.rs/tree/main/examples/jupyter/jupyter_notebook.ipynb).
