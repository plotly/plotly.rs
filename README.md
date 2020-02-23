# Plotly for Rust [![Crate](https://img.shields.io/crates/v/plotly.svg)](https://crates.io/crates/plotly) [![Documentation](https://docs.rs/plotly/badge.svg)](https://docs.rs/plotly) ![Minimum Rust version: 1.30](https://img.shields.io/badge/Minimum%20Rust%20Version-1.30-brightgreen.svg)

A plotting library for Rust powered by [Plotly JS](https://plot.ly/javascript/).


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
plotly = "0.3.0"
```

## Plotly in action
### Line and Scatter plot
```rust
extern crate plotly;
use plotly::charts::{Mode, Scatter};
use plotly::Plot;

fn line_and_scatter_plot() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .name("trace1")
        .mode(Mode::Markers);
    let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
        .name("trace2")
        .mode(Mode::Lines);
    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12]).name("trace3");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    plot.show();
}

fn main() -> std::io::Result<()> {
    line_and_scatter_plot();
    Ok(())
}
```
![Image](docs/images/line_and_scatter_plot.png)

```rust 
extern crate plotly;
use plotly::charts::{Line, LineShape, Legend, Font};
use plotly::charts::Layout;
use plotly::charts::{Mode, Scatter};
use plotly::Plot;

fn line_shape_options_for_interpolation() {
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
    let layout = Layout::new()
        .legend(Legend::new().y(0.5).trace_order("reversed")
            .font(Font::new().size(16)));
    plot.add_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);
    plot.show();
}

fn main() -> std::io::Result<()> {
    line_shape_options_for_interpolation();
    Ok(())
}
```
![line shape options for interpolation](docs/images/line_shape_options_for_interpolation.png)
[**Click to see interactive version**](docs/images/line_dash.html)

### Bar plot


### Histogram plot


### Candlestick plot


### OHLC plot


### Surface plot



# License

Plotly for Rust is distributed under the terms of both the MIT license.

See [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for details.

![Image](gbm_simple_ohlc_chart.html)