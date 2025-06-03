#![allow(dead_code)]

use ndarray::{Array, Ix1, Ix2};
use plotly::common::Mode;
use plotly::ndarray::ArrayTraces;
use plotly::{Plot, Scatter};

fn single_ndarray_trace(show: bool, file_name: &str) {
    let n: usize = 11;
    let t: Array<f64, Ix1> = Array::range(0., 10., 10. / n as f64);
    let ys: Array<f64, Ix1> = t.iter().map(|v| (*v).powf(2.)).collect();

    let trace = Scatter::from_array(t, ys).mode(Mode::LinesMarkers);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

fn multiple_ndarray_traces_over_columns(show: bool, file_name: &str) {
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

fn multiple_ndarray_traces_over_rows(show: bool, file_name: &str) {
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
            .to_traces(t, ys, ArrayTraces::OverRows);

    let mut plot = Plot::new();
    plot.add_traces(traces);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

fn write_example_to_html(plot: &Plot, name: &str) -> String {
    std::fs::create_dir_all("./output").unwrap();
    let path = format!("./output/{}.html", name);
    plot.write_html(&path);
    path
}

fn main() {
    // Change false to true on any of these lines to display the example.
    single_ndarray_trace(false, "single_ndarray_trace");
    multiple_ndarray_traces_over_columns(false, "multiple_ndarray_traces_over_columns");
    multiple_ndarray_traces_over_rows(false, "multiple_ndarray_traces_over_rows");
}
