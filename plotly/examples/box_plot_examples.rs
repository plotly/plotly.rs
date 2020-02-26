use plotly::box_plot::{BoxMean, BoxPoints};
use plotly::common::{Line, Marker, Orientation, Title};
use plotly::layout::{Axis, BoxMode, Margin};
use plotly::{BoxPlot, Layout, Plot, Rgb, Rgba};
use rand_distr::{Distribution, Uniform};

fn basic_box_plot() {
    let mut rng = rand::thread_rng();
    let uniform1 = Uniform::new(0.0, 1.0);
    let uniform2 = Uniform::new(1.0, 2.0);
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
    plot.show();
}

fn box_plot_that_displays_the_underlying_data() {
    let trace1 = BoxPlot::new(vec![0, 1, 1, 2, 3, 5, 8, 13, 21])
        .box_points(BoxPoints::All)
        .jitter(0.3)
        .point_pos(-1.8);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.show();
}

fn horizontal_box_plot() {
    let trace1 = BoxPlot::new(vec![1, 2, 3, 4, 4, 4, 8, 9, 10]).name("Set 1");
    let trace2 = BoxPlot::new(vec![2, 3, 3, 3, 3, 5, 6, 6, 7]).name("Set 2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.show();
}

fn grouped_box_plot() {
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
        .yaxis(
            Axis::new()
                .title(Title::new("normalized moisture"))
                .zero_line(false),
        )
        .box_mode(BoxMode::Group);

    plot.set_layout(layout);
    plot.show();
}

fn box_plot_styling_outliers() {
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
    let trace4 = BoxPlot::new(y.clone())
        .name("Whiskers and Outliers")
        .marker(Marker::new().color(Rgb::new(107, 174, 214)))
        .box_points(BoxPoints::Outliers);

    let layout = Layout::new().title(Title::new("Box Plot Styling Outliers"));

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.show();
}

fn box_plot_styling_mean_and_standard_deviation() {
    let y = vec![
        2.37, 2.16, 4.82, 1.73, 1.04, 0.23, 1.32, 2.91, 0.11, 4.51, 0.51, 3.75, 1.35, 2.98, 4.50,
        0.18, 4.66, 1.30, 2.06, 1.19,
    ];

    let trace1 = BoxPlot::new(y.clone())
        .name("Only Mean")
        .marker(Marker::new().color(Rgb::new(8, 81, 156)))
        .box_mean(BoxMean::True);
    let trace2 = BoxPlot::new(y.clone())
        .name("Mean and Standard Deviation")
        .marker(Marker::new().color(Rgb::new(8, 81, 156)))
        .box_mean(BoxMean::StandardDeviation);
    let layout = Layout::new().title(Title::new("Box Plot Styling Mean and Standard Deviation"));

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.show();
}

fn grouped_horizontal_box_plot() {
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
        .title(Title::new("Grouped Horizontal Box Plot"))
        .xaxis(
            Axis::new()
                .title(Title::new("normalized moisture"))
                .zero_line(false),
        )
        .box_mode(BoxMode::Group);

    plot.set_layout(layout);
    plot.show();
}

fn fully_styled_box_plot() {
    let rnd_sample = |num, mul| -> Vec<f64> {
        let mut v: Vec<f64> = Vec::with_capacity(num);
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new(0.0, mul);
        for _ in 0..num {
            v.push(uniform.sample(&mut rng));
        }
        v
    };

    let x_data = vec![
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
        .title(Title::new(
            "Points Scored by the Top 9 Scoring NBA Players in 2012",
        ))
        .yaxis(
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
    plot.show();
}

fn main() -> std::io::Result<()> {
    basic_box_plot();
    box_plot_that_displays_the_underlying_data();
    horizontal_box_plot();
    grouped_box_plot();
    box_plot_styling_outliers();
    box_plot_styling_mean_and_standard_deviation();
    grouped_horizontal_box_plot();
    fully_styled_box_plot();
    Ok(())
}
