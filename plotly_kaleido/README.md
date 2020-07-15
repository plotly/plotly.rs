# Plotly Kaleido

Plotly Kaleido implements the `kaleido` feature for [Plotly.rs](https://github.com/igiagkiozis/plotly)
 
The `kaleido` feature enables `Plot` conversion to the following output formats: `png`, `jpeg`, `webp`, `svg`, `pdf` and `eps`. 

## Examples 
 
```rust
extern crate plotly;
use plotly::common::Mode;
use plotly::{Plot, Scatter, ImageFormat};

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

    // The following will save the plot in all available formats and show the plot.
    plot.save("scatter", ImageFormat::PNG,  1024, 680, 1.0);
    plot.show();
}

fn main() -> std::io::Result<()> {
    line_and_scatter_plot();
    Ok(())
}
```
