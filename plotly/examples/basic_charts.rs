use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::{Bar, NamedColor, Plot, Rgb, Rgba, Scatter};
use rand_distr::{Distribution, Normal, Uniform};

// Scatter Plots
fn simple_scatter_plot(show: bool) {
    let n: usize = 100;
    let t: Vec<f64> = linspace(0., 10., n).collect();
    let y = t.iter().map(|x| x.sin()).collect();

    let trace = Scatter::new(t, y).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("simple_scatter_plot")));
}

fn line_and_scatter_plots(show: bool) {
    let n: usize = 100;
    let rng = rand::thread_rng();
    let random_x: Vec<f64> = linspace(0., 1., n).collect();
    let random_y0: Vec<f64> = Normal::new(5., 1.)
        .unwrap()
        .sample_iter(rng)
        .take(n)
        .collect();
    let random_y1: Vec<f64> = Normal::new(0., 1.)
        .unwrap()
        .sample_iter(rng)
        .take(n)
        .collect();
    let random_y2: Vec<f64> = Normal::new(-5., 1.)
        .unwrap()
        .sample_iter(rng)
        .take(n)
        .collect();

    let trace1 = Scatter::new(random_x.clone(), random_y0)
        .mode(Mode::Markers)
        .name("markers");
    let trace2 = Scatter::new(random_x.clone(), random_y1)
        .mode(Mode::LinesMarkers)
        .name("linex+markers");
    let trace3 = Scatter::new(random_x, random_y2)
        .mode(Mode::Lines)
        .name("lines");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("line_and_scatter_plots")));
}

fn bubble_scatter_plots(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 11, 12, 13])
        .mode(Mode::Markers)
        .marker(
            Marker::new()
                .size_array(vec![40, 60, 80, 100])
                .color_array(vec![
                    NamedColor::Red,
                    NamedColor::Blue,
                    NamedColor::Cyan,
                    NamedColor::OrangeRed,
                ]),
        );
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("bubble_scatter_plots")));
}

fn data_labels_hover(show: bool) {
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
        .x_axis(Axis::new().title(Title::new("x")).range(vec![0.75, 5.25]))
        .y_axis(Axis::new().title(Title::new("y")).range(vec![0., 8.]));
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("data_labels_hover")));
}

fn data_labels_on_the_plot(show: bool) {
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
        .x_axis(Axis::new().range(vec![0.75, 5.25]))
        .y_axis(Axis::new().range(vec![0., 8.]));
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("data_labels_on_the_plot")));
}

fn colored_and_styled_scatter_plot(show: bool) {
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
        .x_axis(
            Axis::new()
                .title(Title::new("GDP per Capita"))
                .show_grid(false)
                .zero_line(false),
        )
        .y_axis(Axis::new().title(Title::new("Percent")).show_line(false));
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("colored_and_styled_scatter_plot"))
    );
}

fn large_data_sets(show: bool) {
    let n: usize = 100_000;
    let rng = rand::thread_rng();
    let r: Vec<f64> = Uniform::new(0., 1.).sample_iter(rng).take(n).collect();
    let theta: Vec<f64> = Normal::new(0., 2. * std::f64::consts::PI)
        .unwrap()
        .sample_iter(rng)
        .take(n)
        .collect();
    let colors: Vec<f64> = Normal::new(0., 1.)
        .unwrap()
        .sample_iter(rng)
        .take(n)
        .collect();

    let x: Vec<f64> = r
        .iter()
        .zip(theta.iter())
        .map(|args| args.0 * args.1.cos())
        .collect();
    let y: Vec<f64> = r
        .iter()
        .zip(theta.iter())
        .map(|args| args.0 * args.1.sin())
        .collect();
    let trace = Scatter::new(x, y)
        .open_gl_mode(true)
        .mode(Mode::Markers)
        .marker(
            Marker::new()
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                .color_array(colors)
                .line(Line::new().width(1.)),
        );
    let mut plot = Plot::new();
    plot.add_trace(trace);

    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("large_data_sets")));
}

// Line Charts
fn adding_names_to_line_and_scatter_plot(show: bool) {
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
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("adding_names_to_line_and_scatter_plot"))
    );
}

fn line_and_scatter_styling(show: bool) {
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

    let layout = Layout::new().title(Title::new("Line and Scatter Styling"));
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("line_and_scatter_styling")));
}

fn styling_line_plot(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .mode(Mode::Markers)
        .name("Red")
        .line(Line::new().color(Rgb::new(219, 64, 82)).width(3.0));
    let trace2 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
        .mode(Mode::LinesMarkers)
        .name("Blue")
        .line(Line::new().color(Rgb::new(55, 128, 191)).width(1.0));

    let layout = Layout::new()
        .title(Title::new("Styling Line Plot"))
        .width(500)
        .height(500);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("styling_line_plot")));
}

fn line_shape_options_for_interpolation(show: bool) {
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
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("line_shape_options_for_interpolation"))
    );
}

fn line_dash(show: bool) {
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
        .x_axis(Axis::new().range(vec![0.95, 5.05]).auto_range(false))
        .y_axis(Axis::new().range(vec![0.0, 28.5]).auto_range(false));
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("line_dash")));
}

fn filled_lines(show: bool) {
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
        .x_axis(
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
        .y_axis(
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
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("filled_lines")));
}

// Bar Charts
fn basic_bar_chart(show: bool) {
    let animals = vec!["giraffes", "orangutans", "monkeys"];
    let t = Bar::new(animals, vec![20, 14, 23]);
    let mut plot = Plot::new();
    plot.add_trace(t);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("basic_bar_chart")));
}

fn grouped_bar_chart(show: bool) {
    let animals1 = vec!["giraffes", "orangutans", "monkeys"];
    let trace1 = Bar::new(animals1, vec![20, 14, 23]).name("SF Zoo");

    let animals2 = vec!["giraffes", "orangutans", "monkeys"];
    let trace2 = Bar::new(animals2, vec![12, 18, 29]).name("LA Zoo");

    let layout = Layout::new().bar_mode(BarMode::Group);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("grouped_bar_chart")));
}

fn stacked_bar_chart(show: bool) {
    let animals1 = vec!["giraffes", "orangutans", "monkeys"];
    let trace1 = Bar::new(animals1, vec![20, 14, 23]).name("SF Zoo");

    let animals2 = vec!["giraffes", "orangutans", "monkeys"];
    let trace2 = Bar::new(animals2, vec![12, 18, 29]).name("LA Zoo");

    let layout = Layout::new().bar_mode(BarMode::Stack);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("stacked_bar_chart")));
}

fn main() -> std::io::Result<()> {
    // Scatter Plots
    simple_scatter_plot(true);
    line_and_scatter_plots(true);
    bubble_scatter_plots(true);
    data_labels_hover(true);
    data_labels_on_the_plot(true);
    colored_and_styled_scatter_plot(true);
    large_data_sets(true);

    // Line Charts
    adding_names_to_line_and_scatter_plot(true);
    line_and_scatter_styling(true);
    styling_line_plot(true);
    line_shape_options_for_interpolation(true);
    line_dash(true);
    filled_lines(true);

    // Bar Charts
    basic_bar_chart(true);
    grouped_bar_chart(true);
    stacked_bar_chart(true);
    Ok(())
}
