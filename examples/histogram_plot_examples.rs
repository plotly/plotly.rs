use plotly::charts::Layout;
use plotly::charts::{BarMode, Color, Dim, Histogram, Marker};
use plotly::Plot;
use rand::Rng;

fn sample_normal_distribution(n: usize, mean: f64, std_dev: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Normal::new(mean, std_dev);
    let mut v = Vec::<f64>::with_capacity(n);
    for _idx in 1..n {
        v.push(rng.sample(dist));
    }
    v
}

fn basic_histogram() {
    let samples = sample_normal_distribution(10_000, 0.0, 1.0);
    let trace = Histogram::new(samples).name("h");
    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();
}

fn horizontal_histogram() {
    let samples = sample_normal_distribution(10_000, 0.0, 1.0);
    let trace = Histogram::new_horizontal(samples)
        .name("h")
        .marker(Marker::new().color(Dim::Scalar(Color::Pink)));
    let mut plot = Plot::new();

    plot.add_trace(trace);

    plot.show();
}

fn overlaid_histogram() {
    let samples1 = sample_normal_distribution(500, 0.0, 1.0);
    let trace1 = Histogram::new(samples1)
        .name("trace 1")
        .opacity(0.5)
        .marker(Marker::new().color(Dim::Scalar(Color::Green)));

    let samples2 = sample_normal_distribution(500, 0.0, 1.0);
    let trace2 = Histogram::new(samples2)
        .name("trace 2")
        .opacity(0.6)
        .marker(Marker::new().color(Dim::Scalar(Color::Red)));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().bar_mode(BarMode::Overlay);
    plot.add_layout(layout);
    plot.show();
}

fn stacked_histograms() {
    let samples1 = sample_normal_distribution(500, 0.0, 1.0);
    let trace1 = Histogram::new(samples1)
        .name("trace 1")
        .opacity(0.5)
        .marker(Marker::new().color(Dim::Scalar(Color::Green)));

    let samples2 = sample_normal_distribution(500, 0.0, 1.0);
    let trace2 = Histogram::new(samples2)
        .name("trace 2")
        .opacity(0.6)
        .marker(Marker::new().color(Dim::Scalar(Color::Red)));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().bar_mode(BarMode::Stack);
    plot.add_layout(layout);

    plot.show();
}

fn main() -> std::io::Result<()> {
    basic_histogram();
    horizontal_histogram();
    overlaid_histogram();
    stacked_histograms();

    Ok(())
}
