use plotly::common::{
    DashType, ErrorData, ErrorType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, Layout, Legend, TicksDirection};
use plotly::{NamedColor, Plot, Rgb, Rgba, Scatter};
use rand_distr::{Distribution, Normal};

fn geometric_brownian_motion(s_0: f64, dt: f64, n: usize, drift: f64, diffusion: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let dist = Normal::new(0.0, 1.0).unwrap();
    let mut v = Vec::<f64>::with_capacity(n);
    v.push(s_0);
    let drift_factor = 1.0 + drift * dt;
    let diffusion_factor = diffusion * dt.sqrt();
    for idx in 1..n {
        let rv = drift_factor + diffusion_factor * dist.sample(&mut rng);
        let prod: f64 = rv * v[idx - 1];
        v.push(prod);
    }
    v
}

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

fn adding_names_to_line_and_scatter_plot() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .mode(Mode::Markers)
        .name("Scatter");
    let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
        .mode(Mode::Lines)
        .name("Lines");
    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
        .mode(Mode::LinesMarkers)
        .name("Scatter + Lines");

    let layout = Layout::new().title(Title::new("Adding Names to Line and Scatter Plot"));
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.set_layout(layout);
    plot.show();
}

fn line_and_scatter_styling() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .mode(Mode::Markers)
        .name("trace1")
        .marker(Marker::new().color(Rgb::new(219, 64, 82)).size(12));
    let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
        .mode(Mode::Lines)
        .name("trace2")
        .line(Line::new().color(Rgb::new(55, 128, 191)).width(3.0));
    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
        .mode(Mode::LinesMarkers)
        .name("trace3")
        .marker(Marker::new().color(Rgb::new(128, 0, 128)).size(12))
        .line(Line::new().color(Rgb::new(128, 0, 128)).width(1.0));

    let layout = Layout::new().title(Title::new("Adding Names to Line and Scatter Plot"));
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.set_layout(layout);
    plot.show();
}

fn styling_line_plot() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .mode(Mode::Markers)
        .name("Red")
        .line(Line::new().color(Rgb::new(219, 64, 82)).width(3.0));
    let trace2 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
        .mode(Mode::LinesMarkers)
        .name("Blue")
        .line(Line::new().color(Rgb::new(55, 128, 191)).width(1.0));

    let layout = Layout::new()
        .title(Title::new("Adding Names to Line and Scatter Plot"))
        .width(500)
        .height(500);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);
    plot.show();
}

fn colored_and_styled_scatter_plot() {
    let trace1 = Scatter::new(vec![52698, 43117], vec![53, 31])
        .mode(Mode::Markers)
        .name("North America")
        .text_array(vec!["United States", "Canada"])
        .marker(
            Marker::new()
                .color(Rgb::new(164, 194, 244))
                .size(12)
                .line(Line::new().color(NamedColor::White).width(0.5)),
        );
    let trace2 = Scatter::new(
        vec![
            39317, 37236, 35650, 30066, 29570, 27159, 23557, 21046, 18007,
        ],
        vec![33, 20, 13, 19, 27, 19, 49, 44, 38],
    )
    .mode(Mode::Markers)
    .name("Europe")
    .text_array(vec![
        "Germany",
        "Britain",
        "France",
        "Spain",
        "Italy",
        "Czech Rep.",
        "Greece",
        "Poland",
    ])
    .marker(Marker::new().color(Rgb::new(255, 217, 102)).size(12));
    let trace3 = Scatter::new(
        vec![42952, 37037, 33106, 17478, 9813, 5253, 4692, 3899],
        vec![23, 42, 54, 89, 14, 99, 93, 70],
    )
    .mode(Mode::Markers)
    .name("Asia/Pacific")
    .text_array(vec![
        "Australia",
        "Japan",
        "South Korea",
        "Malaysia",
        "China",
        "Indonesia",
        "Philippines",
        "India",
    ])
    .marker(Marker::new().color(Rgb::new(234, 153, 153)).size(12));
    let trace4 = Scatter::new(
        vec![19097, 18601, 15595, 13546, 12026, 7434, 5419],
        vec![43, 47, 56, 80, 86, 93, 80],
    )
    .mode(Mode::Markers)
    .name("Latin America")
    .text_array(vec![
        "Chile",
        "Argentina",
        "Mexico",
        "Venezuela",
        "Venezuela",
        "El Salvador",
        "Bolivia",
    ])
    .marker(Marker::new().color(Rgb::new(142, 124, 195)).size(12));

    let layout = Layout::new()
        .title(Title::new("Quarter 1 Growth"))
        .xaxis(
            Axis::new()
                .title(Title::new("GDP per Capita"))
                .show_grid(false)
                .zero_line(false),
        )
        .yaxis(Axis::new().title(Title::new("Percent")).show_line(false));
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.set_layout(layout);
    plot.show_png(1024, 680);
    plot.show();
}

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
    let layout = Layout::new().legend(
        Legend::new()
            .y(0.5)
            .trace_order("reversed")
            .font(Font::new().size(16)),
    );
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);
    plot.show_png(1024, 680);
    plot.show();
}

fn line_dash() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4, 5], vec![1, 3, 2, 3, 1])
        .mode(Mode::LinesMarkers)
        .name("solid")
        .line(Line::new().dash(DashType::Solid));
    let trace2 = Scatter::new(vec![1, 2, 3, 4, 5], vec![6, 8, 7, 8, 6])
        .mode(Mode::LinesMarkers)
        .name("dashdot")
        .line(Line::new().dash(DashType::DashDot));
    let trace3 = Scatter::new(vec![1, 2, 3, 4, 5], vec![11, 13, 12, 13, 11])
        .mode(Mode::LinesMarkers)
        .name("dash")
        .line(Line::new().dash(DashType::Dash));
    let trace4 = Scatter::new(vec![1, 2, 3, 4, 5], vec![16, 18, 17, 18, 16])
        .mode(Mode::LinesMarkers)
        .name("dot")
        .line(Line::new().dash(DashType::Dot));
    let trace5 = Scatter::new(vec![1, 2, 3, 4, 5], vec![21, 23, 22, 23, 21])
        .mode(Mode::LinesMarkers)
        .name("longdash")
        .line(Line::new().dash(DashType::LongDash));
    let trace6 = Scatter::new(vec![1, 2, 3, 4, 5], vec![26, 28, 27, 28, 26])
        .mode(Mode::LinesMarkers)
        .name("longdashdot")
        .line(Line::new().dash(DashType::LongDashDot));

    let mut plot = Plot::new();
    let layout = Layout::new()
        .legend(
            Legend::new()
                .y(0.5)
                .trace_order("reversed")
                .font(Font::new().size(16)),
        )
        .xaxis(Axis::new().range(vec![0.95, 5.05]).auto_range(false))
        .yaxis(Axis::new().range(vec![0.0, 28.5]).auto_range(false));
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);
    plot.show();
}

fn data_labels_hover() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4, 5], vec![1, 6, 3, 6, 1])
        .mode(Mode::Markers)
        .name("Team A")
        .marker(Marker::new().size(12));
    let trace2 = Scatter::new(vec![1.5, 2.5, 3.5, 4.5, 5.5], vec![4, 1, 7, 1, 4])
        .mode(Mode::Markers)
        .name("Team B")
        .marker(Marker::new().size(12));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .title(Title::new("Data Labels Hover"))
        .xaxis(Axis::new().title(Title::new("x")).range(vec![0.75, 5.25]))
        .yaxis(Axis::new().title(Title::new("y")).range(vec![0., 8.]));
    plot.set_layout(layout);
    plot.show();
}

fn data_labels_on_the_plot() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4, 5], vec![1, 6, 3, 6, 1])
        .mode(Mode::Markers)
        .name("Team A")
        .marker(Marker::new().size(12))
        .text_array(vec!["A-1", "A-2", "A-3", "A-4", "A-5"]);
    let trace2 = Scatter::new(vec![1.5, 2.5, 3.5, 4.5, 5.5], vec![4, 1, 7, 1, 4])
        .mode(Mode::Markers)
        .name("Team B")
        .text_array(vec!["B-a", "B-b", "B-c", "B-d", "B-e"])
        .marker(Marker::new().size(12));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .title(Title::new("Data Labels on the Plot"))
        .xaxis(Axis::new().range(vec![0.75, 5.25]))
        .yaxis(Axis::new().range(vec![0., 8.]));
    plot.set_layout(layout);
    plot.show();
}

// fn labelling_lines_with_annotations() {
//
// }

fn gbm_scatter_plot() {
    let n = 3_000;
    let x: Vec<_> = (0..n).collect();
    let y = geometric_brownian_motion(100.0, 1.0 / 365.0, n, 0.15, 0.5);
    let t = Scatter::new(x, y).name("path_0");
    let mut plot = Plot::new();
    plot.add_trace(t);
    plot.show();
}

fn basic_symmetric_error_bars() {
    let trace1 = Scatter::new(vec![0, 1, 2], vec![6, 10, 2])
        .name("trace1")
        .error_y(ErrorData::new(ErrorType::Data).array(vec![1.0, 2.0, 3.0]));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.show();
}

fn filled_lines() {
    let x1 = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0,
        2.0, 1.0,
    ];
    let x2 = (1..=10).map(|iv| iv as f64).collect::<Vec<f64>>();
    let trace1 = Scatter::new(
        x1.clone(),
        vec![
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0,
            2.0, 1.0, 0.0,
        ],
    )
    .fill(Fill::ToZeroX)
    .fill_color(Rgba::new(0, 100, 80, 0.2))
    .line(Line::new().color(NamedColor::Transparent))
    .name("Fair")
    .show_legend(false);
    let trace2 = Scatter::new(
        x1.clone(),
        vec![
            5.5, 3.0, 5.5, 8.0, 6.0, 3.0, 8.0, 5.0, 6.0, 5.5, 4.75, 5.0, 4.0, 7.0, 2.0, 4.0, 7.0,
            4.4, 2.0, 4.5,
        ],
    )
    .fill(Fill::ToZeroX)
    .fill_color(Rgba::new(0, 176, 246, 0.2))
    .line(Line::new().color(NamedColor::Transparent))
    .name("Premium")
    .show_legend(false);
    let trace3 = Scatter::new(
        x1.clone(),
        vec![
            11.0, 9.0, 7.0, 5.0, 3.0, 1.0, 3.0, 5.0, 3.0, 1.0, -1.0, 1.0, 3.0, 1.0, -0.5, 1.0, 3.0,
            5.0, 7.0, 9.0,
        ],
    )
    .fill(Fill::ToZeroX)
    .fill_color(Rgba::new(231, 107, 243, 0.2))
    .line(Line::new().color(NamedColor::Transparent))
    .name("Fair")
    .show_legend(false);
    let trace4 = Scatter::new(
        x2.clone(),
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    )
    .line(Line::new().color(Rgb::new(0, 100, 80)))
    .name("Fair");
    let trace5 = Scatter::new(
        x2.clone(),
        vec![5.0, 2.5, 5.0, 7.5, 5.0, 2.5, 7.5, 4.5, 5.5, 5.0],
    )
    .line(Line::new().color(Rgb::new(0, 176, 246)))
    .name("Premium");
    let trace6 = Scatter::new(
        x2.clone(),
        vec![10.0, 8.0, 6.0, 4.0, 2.0, 0.0, 2.0, 4.0, 2.0, 0.0],
    )
    .line(Line::new().color(Rgb::new(231, 107, 243)))
    .name("Ideal");

    let layout = Layout::new()
        .paper_background_color(Rgb::new(255, 255, 255))
        .plot_background_color(Rgb::new(229, 229, 229))
        .xaxis(
            Axis::new()
                .grid_color(Rgb::new(255, 255, 255))
                .range(vec![1.0, 10.0])
                .show_grid(true)
                .show_line(false)
                .show_tick_labels(true)
                .tick_color(Rgb::new(127, 127, 127))
                .ticks(TicksDirection::Outside)
                .zero_line(false),
        )
        .yaxis(
            Axis::new()
                .grid_color(Rgb::new(255, 255, 255))
                .show_grid(true)
                .show_line(false)
                .show_tick_labels(true)
                .tick_color(Rgb::new(127, 127, 127))
                .ticks(TicksDirection::Outside)
                .zero_line(false),
        );

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);
    plot.show();
}

fn main() -> std::io::Result<()> {
    line_and_scatter_plot();
    data_labels_hover();
    data_labels_on_the_plot();
    gbm_scatter_plot();
    adding_names_to_line_and_scatter_plot();
    line_and_scatter_styling();
    styling_line_plot();
    colored_and_styled_scatter_plot();
    line_shape_options_for_interpolation();
    line_dash();
    basic_symmetric_error_bars();
    filled_lines();
    Ok(())
}
