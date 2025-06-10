#![allow(dead_code)]

use ndarray::Array;
use plotly::{
    box_plot::{BoxMean, BoxPoints},
    color::{NamedColor, Rgb, Rgba},
    common::{ErrorData, ErrorType, Line, Marker, Mode, Orientation},
    histogram::{Bins, Cumulative, HistFunc, HistNorm},
    layout::{Axis, BarMode, BoxMode, Layout, Margin},
    Bar, BoxPlot, Histogram, Plot, Scatter,
};
use rand_distr::{Distribution, Normal, Uniform};

// Error Bars
// ANCHOR: basic_symmetric_error_bars
fn basic_symmetric_error_bars(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![0, 1, 2], vec![6, 10, 2])
        .name("trace1")
        .error_y(ErrorData::new(ErrorType::Data).array(vec![1.0, 2.0, 3.0]));

    let mut plot = Plot::new();
    plot.add_trace(trace1);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: basic_symmetric_error_bars

// ANCHOR: asymmetric_error_bars
fn asymmetric_error_bars(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![2, 1, 3, 4])
        .name("trace1")
        .error_y(
            ErrorData::new(ErrorType::Data)
                .array(vec![0.1, 0.2, 0.1, 0.1])
                .array_minus(vec![0.2, 0.4, 1., 0.2]),
        );

    let mut plot = Plot::new();
    plot.add_trace(trace1);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: asymmetric_error_bars

// ANCHOR: error_bars_as_a_percentage_of_the_y_value
fn error_bars_as_a_percentage_of_the_y_value(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![0, 1, 2], vec![6, 10, 2])
        .name("trace1")
        .error_y(ErrorData::new(ErrorType::Percent).value(50.).visible(true));

    let mut plot = Plot::new();
    plot.add_trace(trace1);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: error_bars_as_a_percentage_of_the_y_value

// ANCHOR: asymmetric_error_bars_with_a_constant_offset
fn asymmetric_error_bars_with_a_constant_offset(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![2, 1, 3, 4])
        .name("trace1")
        .error_y(
            ErrorData::new(ErrorType::Percent)
                .symmetric(false)
                .value(15.)
                .value_minus(25.),
        );

    let mut plot = Plot::new();
    plot.add_trace(trace1);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: asymmetric_error_bars_with_a_constant_offset

// ANCHOR: horizontal_error_bars
fn horizontal_error_bars(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![2, 1, 3, 4])
        .name("trace1")
        .error_x(ErrorData::new(ErrorType::Percent).value(10.));

    let mut plot = Plot::new();
    plot.add_trace(trace1);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: horizontal_error_bars

// ANCHOR: bar_chart_with_error_bars
fn bar_chart_with_error_bars(show: bool, file_name: &str) {
    let trace_c = Bar::new(vec!["Trial 1", "Trial 2", "Trial 3"], vec![3, 6, 4])
        .error_y(ErrorData::new(ErrorType::Data).array(vec![1., 0.5, 1.5]));
    let trace_e = Bar::new(vec!["Trial 1", "Trial 2", "Trial 3"], vec![4, 7, 3])
        .error_y(ErrorData::new(ErrorType::Data).array(vec![0.5, 1., 2.]));

    let mut plot = Plot::new();
    plot.add_trace(trace_c);
    plot.add_trace(trace_e);

    let layout = Layout::new().bar_mode(BarMode::Group);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: bar_chart_with_error_bars

// ANCHOR: colored_and_styled_error_bars
fn colored_and_styled_error_bars(show: bool, file_name: &str) {
    let x_theo: Vec<f64> = Array::linspace(-4., 4., 100).into_raw_vec_and_offset().0;
    let sincx: Vec<f64> = x_theo
        .iter()
        .map(|x| (x * std::f64::consts::PI).sin() / (*x * std::f64::consts::PI))
        .collect();
    let x = vec![
        -3.8, -3.03, -1.91, -1.46, -0.89, -0.24, -0.0, 0.41, 0.89, 1.01, 1.91, 2.28, 2.79, 3.56,
    ];
    let y = vec![
        -0.02, 0.04, -0.01, -0.27, 0.36, 0.75, 1.03, 0.65, 0.28, 0.02, -0.11, 0.16, 0.04, -0.15,
    ];

    let trace1 = Scatter::new(x_theo, sincx).name("sinc(x)");
    let trace2 = Scatter::new(x, y)
        .mode(Mode::Markers)
        .name("measured")
        .error_y(
            ErrorData::new(ErrorType::Constant)
                .value(0.1)
                .color(NamedColor::Purple)
                .thickness(1.5)
                .width(3),
        )
        .error_x(
            ErrorData::new(ErrorType::Constant)
                .value(0.2)
                .color(NamedColor::Purple)
                .thickness(1.5)
                .width(3),
        )
        .marker(Marker::new().color(NamedColor::Purple).size(8));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: colored_and_styled_error_bars

// Box Plots
// ANCHOR: basic_box_plot
fn basic_box_plot(show: bool, file_name: &str) {
    let mut rng = rand::rng();
    let uniform1 = Uniform::new(0.0, 1.0).unwrap();
    let uniform2 = Uniform::new(1.0, 2.0).unwrap();
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: basic_box_plot

// ANCHOR: box_plot_that_displays_the_underlying_data
fn box_plot_that_displays_the_underlying_data(show: bool, file_name: &str) {
    let trace1 = BoxPlot::new(vec![0, 1, 1, 2, 3, 5, 8, 13, 21])
        .box_points(BoxPoints::All)
        .jitter(0.3)
        .point_pos(-1.8);
    let mut plot = Plot::new();
    plot.add_trace(trace1);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: box_plot_that_displays_the_underlying_data

// ANCHOR: horizontal_box_plot
fn horizontal_box_plot(show: bool, file_name: &str) {
    let x = vec![
        "Set 1", "Set 1", "Set 1", "Set 1", "Set 1", "Set 1", "Set 1", "Set 1", "Set 1", "Set 2",
        "Set 2", "Set 2", "Set 2", "Set 2", "Set 2", "Set 2", "Set 2", "Set 2",
    ];

    let trace = BoxPlot::new_xy(
        vec![1, 2, 3, 4, 4, 4, 8, 9, 10, 2, 3, 3, 3, 3, 5, 6, 6, 7],
        x.clone(),
    )
    .orientation(Orientation::Horizontal);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: horizontal_box_plot

// ANCHOR: grouped_box_plot
fn grouped_box_plot(show: bool, file_name: &str) {
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
        .y_axis(Axis::new().title("normalized moisture").zero_line(false))
        .box_mode(BoxMode::Group);

    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: grouped_box_plot

// ANCHOR: box_plot_styling_outliers
fn box_plot_styling_outliers(show: bool, file_name: &str) {
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
    let trace4 = BoxPlot::new(y)
        .name("Whiskers and Outliers")
        .marker(Marker::new().color(Rgb::new(107, 174, 214)))
        .box_points(BoxPoints::Outliers);

    let layout = Layout::new().title("Box Plot Styling Outliers");

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: box_plot_styling_outliers

// ANCHOR: box_plot_styling_mean_and_standard_deviation
fn box_plot_styling_mean_and_standard_deviation(show: bool, file_name: &str) {
    let y = vec![
        2.37, 2.16, 4.82, 1.73, 1.04, 0.23, 1.32, 2.91, 0.11, 4.51, 0.51, 3.75, 1.35, 2.98, 4.50,
        0.18, 4.66, 1.30, 2.06, 1.19,
    ];

    let trace1 = BoxPlot::new(y.clone())
        .name("Only Mean")
        .marker(Marker::new().color(Rgb::new(8, 81, 156)))
        .box_mean(BoxMean::True);
    let trace2 = BoxPlot::new(y)
        .name("Mean and Standard Deviation")
        .marker(Marker::new().color(Rgb::new(8, 81, 156)))
        .box_mean(BoxMean::StandardDeviation);
    let layout = Layout::new().title("Box Plot Styling Mean and Standard Deviation");

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: box_plot_styling_mean_and_standard_deviation

// ANCHOR: grouped_horizontal_box_plot
fn grouped_horizontal_box_plot(show: bool, file_name: &str) {
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
        .title("Grouped Horizontal Box Plot")
        .x_axis(Axis::new().title("normalized moisture").zero_line(false))
        .box_mode(BoxMode::Group);

    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: grouped_horizontal_box_plot

// ANCHOR: fully_styled_box_plot
fn fully_styled_box_plot(show: bool, file_name: &str) {
    let rnd_sample = |num, mul| -> Vec<f64> {
        let mut v: Vec<f64> = Vec::with_capacity(num);
        let mut rng = rand::rng();
        let uniform = Uniform::new(0.0, mul).unwrap();
        for _ in 0..num {
            v.push(uniform.sample(&mut rng));
        }
        v
    };

    let x_data = [
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
        .title("Points Scored by the Top 9 Scoring NBA Players in 2012")
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: fully_styled_box_plot

// Histograms
fn sample_normal_distribution(n: usize, mean: f64, std_dev: f64) -> Vec<f64> {
    let mut rng = rand::rng();
    let dist = Normal::new(mean, std_dev).unwrap();
    let mut v = Vec::<f64>::with_capacity(n);
    for _idx in 1..n {
        v.push(dist.sample(&mut rng));
    }
    v
}

fn sample_uniform_distribution(n: usize, lb: f64, ub: f64) -> Vec<f64> {
    let mut rng = rand::rng();
    let dist = Uniform::new(lb, ub).unwrap();
    let mut v = Vec::<f64>::with_capacity(n);
    for _idx in 1..n {
        v.push(dist.sample(&mut rng));
    }
    v
}

// ANCHOR: basic_histogram
fn basic_histogram(show: bool, file_name: &str) {
    let samples = sample_normal_distribution(10_000, 0.0, 1.0);
    let trace = Histogram::new(samples).name("h");
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: basic_histogram

// ANCHOR: horizontal_histogram
fn horizontal_histogram(show: bool, file_name: &str) {
    let samples = sample_normal_distribution(10_000, 0.0, 1.0);
    let trace = Histogram::new_vertical(samples)
        .name("h")
        .marker(Marker::new().color(NamedColor::Pink));
    let mut plot = Plot::new();

    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: horizontal_histogram

// ANCHOR: overlaid_histogram
fn overlaid_histogram(show: bool, file_name: &str) {
    let samples1 = sample_normal_distribution(500, 0.0, 1.0);
    let trace1 = Histogram::new(samples1)
        .name("trace 1")
        .opacity(0.5)
        .marker(Marker::new().color(NamedColor::Green));

    let samples2 = sample_normal_distribution(500, 0.0, 1.0);
    let trace2 = Histogram::new(samples2)
        .name("trace 2")
        .opacity(0.6)
        .marker(Marker::new().color(NamedColor::Red));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().bar_mode(BarMode::Overlay);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: overlaid_histogram

// ANCHOR: stacked_histograms
fn stacked_histograms(show: bool, file_name: &str) {
    let samples1 = sample_normal_distribution(500, 0.0, 1.0);
    let trace1 = Histogram::new(samples1)
        .name("trace 1")
        .opacity(0.5)
        .marker(Marker::new().color(NamedColor::Green));

    let samples2 = sample_normal_distribution(500, 0.0, 1.0);
    let trace2 = Histogram::new(samples2)
        .name("trace 2")
        .opacity(0.6)
        .marker(Marker::new().color(NamedColor::Red));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().bar_mode(BarMode::Stack);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: stacked_histograms

// ANCHOR: colored_and_styled_histograms
fn colored_and_styled_histograms(show: bool, file_name: &str) {
    let n = 500;
    let x1 = sample_uniform_distribution(n, 0.0, 5.0);
    let x2 = sample_uniform_distribution(n, 0.0, 10.0);
    let y1 = sample_uniform_distribution(n, 0.0, 1.0);
    let y2 = sample_uniform_distribution(n, 0.0, 2.0);

    let trace1 = Histogram::new_xy(x1, y1)
        .name("control")
        .hist_func(HistFunc::Count)
        .marker(
            Marker::new()
                .color(Rgba::new(255, 100, 102, 0.7))
                .line(Line::new().color(Rgba::new(255, 100, 102, 1.0)).width(1.0)),
        )
        .opacity(0.5)
        .auto_bin_x(false)
        .x_bins(Bins::new(0.5, 2.8, 0.06));
    let trace2 = Histogram::new_xy(x2, y2)
        .name("experimental")
        .hist_func(HistFunc::Count)
        .marker(
            Marker::new()
                .color(Rgba::new(100, 200, 102, 0.7))
                .line(Line::new().color(Rgba::new(100, 200, 102, 1.0)).width(1.0)),
        )
        .opacity(0.75)
        .auto_bin_x(false)
        .x_bins(Bins::new(-3.2, 4.0, 0.06));
    let layout = Layout::new()
        .title("Colored and Styled Histograms")
        .x_axis(Axis::new().title("Value"))
        .y_axis(Axis::new().title("Count"))
        .bar_mode(BarMode::Overlay)
        .bar_gap(0.05)
        .bar_group_gap(0.2);

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: colored_and_styled_histograms

// ANCHOR: cumulative_histogram
fn cumulative_histogram(show: bool, file_name: &str) {
    let n = 500;
    let x = sample_uniform_distribution(n, 0.0, 1.0);
    let trace = Histogram::new(x)
        .cumulative(Cumulative::new().enabled(true))
        .marker(Marker::new().color(NamedColor::BurlyWood));
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: cumulative_histogram

// ANCHOR: normalized_histogram
fn normalized_histogram(show: bool, file_name: &str) {
    let n = 500;
    let x = sample_uniform_distribution(n, 0.0, 1.0);
    let trace = Histogram::new(x)
        .hist_norm(HistNorm::Probability)
        .marker(Marker::new().color(NamedColor::SeaGreen));
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: normalized_histogram

// ANCHOR: specify_binning_function
fn specify_binning_function(show: bool, file_name: &str) {
    let x = vec!["Apples", "Apples", "Apples", "Oranges", "Bananas"];
    let y = vec!["5", "10", "3", "10", "5"];

    let trace1 = Histogram::new_xy(x.clone(), y.clone())
        .name("count")
        .hist_func(HistFunc::Count);
    let trace2 = Histogram::new_xy(x.clone(), y.clone())
        .name("sum")
        .hist_func(HistFunc::Sum);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: specify_binning_function

fn write_example_to_html(plot: &Plot, name: &str) -> String {
    std::fs::create_dir_all("./output").unwrap();
    // Write inline HTML
    let html = plot.to_inline_html(Some(&name));
    let path = format!("./output/inline_{}.html", name);
    std::fs::write(path, html).unwrap();
    // Write standalone HTML
    let path = format!("./output/{}.html", name);
    plot.write_html(&path);
    path
}

fn main() {
    // Change false to true on any of these lines to display the example.

    // Error Bars

    basic_symmetric_error_bars(false, "basic_symmetric_error_bars");
    asymmetric_error_bars(false, "asymmetric_error_bars");

    error_bars_as_a_percentage_of_the_y_value(false, "error_bars_as_a_percentage_of_the_y_value");

    asymmetric_error_bars_with_a_constant_offset(
        false,
        "asymmetric_error_bars_with_a_constant_offset",
    );
    horizontal_error_bars(false, "horizontal_error_bars");

    bar_chart_with_error_bars(false, "bar_chart_with_error_bars");

    colored_and_styled_error_bars(false, "colored_and_styled_error_bars");

    // Box Plots
    basic_box_plot(false, "basic_box_plot");

    box_plot_that_displays_the_underlying_data(false, "box_plot_that_displays_the_underlying_data");
    horizontal_box_plot(false, "horizontal_box_plot");
    grouped_box_plot(false, "grouped_box_plot");

    box_plot_styling_outliers(false, "box_plot_styling_outliers");

    box_plot_styling_mean_and_standard_deviation(
        false,
        "box_plot_styling_mean_and_standard_deviation",
    );

    grouped_horizontal_box_plot(false, "grouped_horizontal_box_plot");
    fully_styled_box_plot(false, "fully_styled_box_plot");

    // Histograms
    basic_histogram(false, "basic_histogram");
    horizontal_histogram(false, "horizontal_histogram");
    overlaid_histogram(false, "overlaid_histogram");
    stacked_histograms(false, "stacked_histograms");

    colored_and_styled_histograms(false, "colored_and_styled_histograms");
    cumulative_histogram(false, "cumulative_histogram");
    normalized_histogram(false, "normalized_histogram");
    specify_binning_function(false, "specify_binning_function");
}
