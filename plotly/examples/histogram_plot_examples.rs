use plotly::common::{Line, Marker, Title};
use plotly::histogram::{Bins, Cumulative, HistFunc, HistNorm};
use plotly::layout::{Axis, BarMode, Layout};
use plotly::{Histogram, NamedColor, Plot, Rgba};
use rand_distr::{Distribution, Normal, Uniform};

fn sample_normal_distribution(n: usize, mean: f64, std_dev: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let dist = Normal::new(mean, std_dev).unwrap();
    let mut v = Vec::<f64>::with_capacity(n);
    for _idx in 1..n {
        v.push(dist.sample(&mut rng));
    }
    v
}

fn sample_uniform_distribution(n: usize, lb: f64, ub: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(lb, ub);
    let mut v = Vec::<f64>::with_capacity(n);
    for _idx in 1..n {
        v.push(dist.sample(&mut rng));
    }
    v
}

fn basic_histogram() {
    let samples = sample_normal_distribution(10_000, 0.0, 1.0);
    let trace = Histogram::new(samples).name("h");
    let mut plot = Plot::default();
    plot.add_trace(trace);

    plot.show();
}

fn horizontal_histogram() {
    let samples = sample_normal_distribution(10_000, 0.0, 1.0);
    let trace = Histogram::new_horizontal(samples)
        .name("h")
        .marker(Marker::default().color(NamedColor::Pink));
    let mut plot = Plot::default();

    plot.add_trace(trace);

    plot.show();
}

fn overlaid_histogram() {
    let samples1 = sample_normal_distribution(500, 0.0, 1.0);
    let trace1 = Histogram::new(samples1)
        .name("trace 1")
        .opacity(0.5)
        .marker(Marker::default().color(NamedColor::Green));

    let samples2 = sample_normal_distribution(500, 0.0, 1.0);
    let trace2 = Histogram::new(samples2)
        .name("trace 2")
        .opacity(0.6)
        .marker(Marker::default().color(NamedColor::Red));

    let mut plot = Plot::default();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::default().bar_mode(BarMode::Overlay);
    plot.set_layout(layout);
    plot.show();
}

fn stacked_histograms() {
    let samples1 = sample_normal_distribution(500, 0.0, 1.0);
    let trace1 = Histogram::new(samples1)
        .name("trace 1")
        .opacity(0.5)
        .marker(Marker::default().color(NamedColor::Green));

    let samples2 = sample_normal_distribution(500, 0.0, 1.0);
    let trace2 = Histogram::new(samples2)
        .name("trace 2")
        .opacity(0.6)
        .marker(Marker::default().color(NamedColor::Red));

    let mut plot = Plot::default();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::default().bar_mode(BarMode::Stack);
    plot.set_layout(layout);

    plot.show();
}

fn colored_and_styled_histograms() {
    let n = 500;
    let x1 = sample_uniform_distribution(n, 0.0, 5.0);
    let x2 = sample_uniform_distribution(n, 0.0, 10.0);
    let y1 = sample_uniform_distribution(n, 0.0, 1.0);
    let y2 = sample_uniform_distribution(n, 0.0, 2.0);

    let trace1 = Histogram::new_xy(x1, y1)
        .name("control")
        .hist_func(HistFunc::Count)
        .marker(
            Marker::default()
                .color(Rgba::new(255, 100, 102, 0.7))
                .line(Line::default().color(Rgba::new(255, 100, 102, 1.0)).width(1.0)),
        )
        .opacity(0.5)
        .auto_bin_x(false)
        .x_bins(Bins::new(0.5, 2.8, 0.06));
    let trace2 = Histogram::new_xy(x2, y2)
        .name("experimental")
        .hist_func(HistFunc::Count)
        .marker(
            Marker::default()
                .color(Rgba::new(100, 200, 102, 0.7))
                .line(Line::default().color(Rgba::new(100, 200, 102, 1.0)).width(1.0)),
        )
        .opacity(0.75)
        .auto_bin_x(false)
        .x_bins(Bins::new(-3.2, 4.0, 0.06));
    let layout = Layout::default()
        .title(Title::new("Sampled Results"))
        .xaxis(Axis::default().title(Title::new("Value")))
        .yaxis(Axis::default().title(Title::new("Count")))
        .bar_mode(BarMode::Overlay)
        .bar_gap(0.05)
        .bar_group_gap(0.2);

    let mut plot = Plot::default();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.show();
}

fn cumulative_histogram() {
    let n = 500;
    let x = sample_uniform_distribution(n, 0.0, 1.0);
    let trace = Histogram::new(x)
        .cumulative(Cumulative::default().enabled(true))
        .marker(Marker::default().color(NamedColor::BurlyWood));
    let mut plot = Plot::default();
    plot.add_trace(trace);
    plot.show();
}

fn normalized_histogram() {
    let n = 500;
    let x = sample_uniform_distribution(n, 0.0, 1.0);
    let trace = Histogram::new(x)
        .hist_norm(HistNorm::Probability)
        .marker(Marker::default().color(NamedColor::SeaGreen));
    let mut plot = Plot::default();
    plot.add_trace(trace);
    plot.show();
}

fn specify_binning_function() {
    let x = vec!["Apples", "Apples", "Apples", "Organges", "Bananas"];
    let y = vec!["5", "10", "3", "10", "5"];

    let trace1 = Histogram::new_xy(x.clone(), y.clone())
        .name("count")
        .hist_func(HistFunc::Count);
    let trace2 = Histogram::new_xy(x.clone(), y.clone())
        .name("sum")
        .hist_func(HistFunc::Sum);

    let mut plot = Plot::default();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.show();
}

fn main() -> std::io::Result<()> {
    basic_histogram();
    horizontal_histogram();
    overlaid_histogram();
    stacked_histograms();
    colored_and_styled_histograms();
    cumulative_histogram();
    normalized_histogram();
    specify_binning_function();
    Ok(())
}
