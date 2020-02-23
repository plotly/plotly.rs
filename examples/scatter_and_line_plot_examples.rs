use plotly::charts::Layout;
use plotly::charts::{Axis, DashType, Font, Legend, Line, LineShape, RgbColor};
use plotly::charts::{Color, Dim, ErrorData, ErrorType, Marker, Mode, Scatter, Title};
use plotly::Plot;
use rand::Rng;

fn geometric_brownian_motion(s_0: f64, dt: f64, n: usize, drift: f64, diffusion: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Normal::new(0.0, 1.0);
    let mut v = Vec::<f64>::with_capacity(n);
    v.push(s_0);
    let drift_factor = 1.0 + drift * dt;
    let diffusion_factor = diffusion * dt.sqrt();
    for idx in 1..n {
        let rv = drift_factor + diffusion_factor * rng.sample(dist);
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
    plot.add_layout(layout);
    plot.show();
}

fn line_and_scatter_styling() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .mode(Mode::Markers)
        .name("trace1")
        .marker(
            Marker::new()
                .color(Dim::Scalar(Color::Rgb(RgbColor::new(219, 64, 82))))
                .size(Dim::Scalar(12)),
        );
    let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
        .mode(Mode::Lines)
        .name("trace2")
        .line(
            Line::new()
                .color(Color::Rgb(RgbColor::new(55, 128, 191)))
                .width(3.0),
        );
    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
        .mode(Mode::LinesMarkers)
        .name("trace3")
        .marker(
            Marker::new()
                .color(Dim::Scalar(Color::Rgb(RgbColor::new(128, 0, 128))))
                .size(Dim::Scalar(12)),
        )
        .line(
            Line::new()
                .color(Color::Rgb(RgbColor::new(128, 0, 128)))
                .width(1.0),
        );

    let layout = Layout::new().title(Title::new("Adding Names to Line and Scatter Plot"));
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_layout(layout);
    plot.show();
}

fn styling_line_plot() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .mode(Mode::Markers)
        .name("Red")
        .line(
            Line::new()
                .color(Color::Rgb(RgbColor::new(219, 64, 82)))
                .width(3.0),
        );
    let trace2 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
        .mode(Mode::LinesMarkers)
        .name("Blue")
        .line(
            Line::new()
                .color(Color::Rgb(RgbColor::new(55, 128, 191)))
                .width(1.0),
        );

    let layout = Layout::new()
        .title(Title::new("Adding Names to Line and Scatter Plot"))
        .width(500)
        .height(500);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_layout(layout);
    plot.show();
}

fn colored_and_styled_scatter_plot() {
    let trace1 = Scatter::new(vec![52698, 43117], vec![53, 31])
        .mode(Mode::Markers)
        .name("North America")
        .text(vec!["United States".to_owned(), "Canada".to_owned()])
        .marker(
            Marker::new()
                .color(Dim::Scalar(Color::Rgb(RgbColor::new(164, 194, 244))))
                .size(Dim::Scalar(12))
                .line(Line::new().color(Color::White).width(0.5)),
        );
    let trace2 = Scatter::new(
        vec![
            39317, 37236, 35650, 30066, 29570, 27159, 23557, 21046, 18007,
        ],
        vec![33, 20, 13, 19, 27, 19, 49, 44, 38],
    )
    .mode(Mode::Markers)
    .name("Europe")
    .text(vec![
        "Germany".to_owned(),
        "Britain".to_owned(),
        "France".to_owned(),
        "Spain".to_owned(),
        "Italy".to_owned(),
        "Czech Rep.".to_owned(),
        "Greece".to_owned(),
        "Poland".to_owned(),
    ])
    .marker(
        Marker::new()
            .color(Dim::Scalar(Color::Rgb(RgbColor::new(255, 217, 102))))
            .size(Dim::Scalar(12)),
    );
    let trace3 = Scatter::new(
        vec![42952, 37037, 33106, 17478, 9813, 5253, 4692, 3899],
        vec![23, 42, 54, 89, 14, 99, 93, 70],
    )
    .mode(Mode::Markers)
    .name("Asia/Pacific")
    .text(vec![
        "Australia".to_owned(),
        "Japan".to_owned(),
        "South Korea".to_owned(),
        "Malaysia".to_owned(),
        "China".to_owned(),
        "Indonesia".to_owned(),
        "Philippines".to_owned(),
        "India".to_owned(),
    ])
    .marker(
        Marker::new()
            .color(Dim::Scalar(Color::Rgb(RgbColor::new(234, 153, 153))))
            .size(Dim::Scalar(12)),
    );
    let trace4 = Scatter::new(
        vec![19097, 18601, 15595, 13546, 12026, 7434, 5419],
        vec![43, 47, 56, 80, 86, 93, 80],
    )
    .mode(Mode::Markers)
    .name("Latin America")
    .text(vec![
        "Chile".to_owned(),
        "Argentina".to_owned(),
        "Mexico".to_owned(),
        "Venezuela".to_owned(),
        "Venezuela".to_owned(),
        "El Salvador".to_owned(),
        "Bolivia".to_owned(),
    ])
    .marker(
        Marker::new()
            .color(Dim::Scalar(Color::Rgb(RgbColor::new(142, 124, 195))))
            .size(Dim::Scalar(12)),
    );

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
    plot.add_layout(layout);
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
    plot.add_layout(layout);
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
    plot.add_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);
    plot.show_png(1024, 680);
    plot.show();
}

fn data_labels_hover() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4, 5], vec![1, 6, 3, 6, 1])
        .mode(Mode::Markers)
        .name("Team A")
        .marker(Marker::new().size(Dim::Scalar(12)));
    let trace2 = Scatter::new(vec![1.5, 2.5, 3.5, 4.5, 5.5], vec![4, 1, 7, 1, 4])
        .mode(Mode::Markers)
        .name("Team B")
        .marker(Marker::new().size(Dim::Scalar(12)));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .title(Title::new("Data Labels Hover"))
        .xaxis(Axis::new().title(Title::new("x")).range(vec![0.75, 5.25]))
        .yaxis(Axis::new().title(Title::new("y")).range(vec![0., 8.]));
    plot.add_layout(layout);

    plot.show();
}

fn data_labels_on_the_plot() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4, 5], vec![1, 6, 3, 6, 1])
        .mode(Mode::Markers)
        .name("Team A")
        .marker(Marker::new().size(Dim::Scalar(12)))
        .text(vec![
            "A-1".to_owned(),
            "A-2".to_owned(),
            "A-3".to_owned(),
            "A-4".to_owned(),
            "A-5".to_owned(),
        ]);
    let trace2 = Scatter::new(vec![1.5, 2.5, 3.5, 4.5, 5.5], vec![4, 1, 7, 1, 4])
        .mode(Mode::Markers)
        .name("Team B")
        .text(vec![
            "B-a".to_owned(),
            "B-b".to_owned(),
            "B-c".to_owned(),
            "B-d".to_owned(),
            "B-e".to_owned(),
        ])
        .marker(Marker::new().size(Dim::Scalar(12)));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .title(Title::new("Data Labels on the Plot"))
        .xaxis(Axis::new().range(vec![0.75, 5.25]))
        .yaxis(Axis::new().range(vec![0., 8.]));
    plot.add_layout(layout);

    plot.show();
}

fn gbm_scatter_plot() {
    let n = 3_000;
    let x = (0..n).collect();
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
    Ok(())
}
