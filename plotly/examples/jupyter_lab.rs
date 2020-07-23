use chrono::prelude::*;
use chrono::{Date, DateTime, Duration, TimeZone, Utc};
use plotly::common::Mode;
use plotly::{Bar, Plot, Scatter};
use plotly_ndarray::{Array, ArrayTraces, Ix1, Ix2};
use std::ops::Add;

fn demo_bar_chart(show: bool) {
    let trace = Bar::new(vec!["a", "b", "c"], vec![11, 22, 33]);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("jupyter_lab_demo_bar_chart_python"))
    );
}

fn main() -> std::io::Result<()> {
    demo_bar_chart(true);

    Ok(())
}
