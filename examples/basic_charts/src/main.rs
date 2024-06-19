#![allow(dead_code)]

use ndarray::Array;
use plotly::{
    color::{NamedColor, Rgb, Rgba},
    common::{
        ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode,
        Orientation, Title,
    },
    layout::{Axis, BarMode, Layout, Legend, TicksDirection, TraceOrder},
    sankey::{Line as SankeyLine, Link, Node},
    traces::table::{Cells, Header},
    Bar, Plot, Sankey, Scatter, ScatterPolar, Table,
};
use rand_distr::{Distribution, Normal, Uniform};

// Scatter Plots
fn simple_scatter_plot() {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec();
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();

    let trace = Scatter::new(t, y).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();
}

fn line_and_scatter_plots() {
    let n: usize = 100;
    let mut rng = rand::thread_rng();
    let random_x: Vec<f64> = Array::linspace(0., 1., n).into_raw_vec();
    let random_y0: Vec<f64> = Normal::new(5., 1.)
        .unwrap()
        .sample_iter(&mut rng)
        .take(n)
        .collect();
    let random_y1: Vec<f64> = Normal::new(0., 1.)
        .unwrap()
        .sample_iter(&mut rng)
        .take(n)
        .collect();
    let random_y2: Vec<f64> = Normal::new(-5., 1.)
        .unwrap()
        .sample_iter(&mut rng)
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

    plot.show();
}

fn bubble_scatter_plots() {
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

    plot.show();
}

fn polar_scatter_plot() {
    let n: usize = 400;
    let theta: Vec<f64> = Array::linspace(0., 360., n).into_raw_vec();
    let r: Vec<f64> = theta
        .iter()
        .map(|x| {
            let x = x / 360. * core::f64::consts::TAU;
            let x = x.cos();
            1. / 8. * (63. * x.powf(5.) - 70. * x.powf(3.) + 15. * x).abs()
        })
        .collect();

    let trace = ScatterPolar::new(theta, r).mode(Mode::Lines);
    let mut plot = Plot::new();
    plot.add_trace(trace);

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
        .title("Data Labels Hover".into())
        .x_axis(Axis::new().title("x".into()).range(vec![0.75, 5.25]))
        .y_axis(Axis::new().title("y".into()).range(vec![0., 8.]));
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
        .title("Data Labels on the Plot".into())
        .x_axis(Axis::new().range(vec![0.75, 5.25]))
        .y_axis(Axis::new().range(vec![0., 8.]));
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

    plot.show();
}

fn large_data_sets() {
    let n: usize = 100_000;
    let mut rng = rand::thread_rng();
    let r: Vec<f64> = Uniform::new(0., 1.).sample_iter(&mut rng).take(n).collect();
    let theta: Vec<f64> = Normal::new(0., 2. * std::f64::consts::PI)
        .unwrap()
        .sample_iter(&mut rng)
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
        .web_gl_mode(true)
        .mode(Mode::Markers)
        .marker(
            Marker::new()
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                .line(Line::new().width(1.)),
        );
    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();
}

// Line Charts
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

    let layout = Layout::new().title(Title::new("Line and Scatter Styling"));
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
        .title(Title::new("Styling Line Plot"))
        .width(500)
        .height(500);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

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
            .trace_order(TraceOrder::Reversed)
            .font(Font::new().size(16)),
    );
    plot.set_layout(layout);
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_trace(trace5);
    plot.add_trace(trace6);

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
                .trace_order(TraceOrder::Reversed)
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
        x1,
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
    let trace6 = Scatter::new(x2, vec![10.0, 8.0, 6.0, 4.0, 2.0, 0.0, 2.0, 4.0, 2.0, 0.0])
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

    plot.show();
}

// Bar Charts
fn basic_bar_chart() {
    let animals = vec!["giraffes", "orangutans", "monkeys"];
    let t = Bar::new(animals, vec![20, 14, 23]);
    let mut plot = Plot::new();
    plot.add_trace(t);

    plot.show();
}

fn grouped_bar_chart() {
    let animals1 = vec!["giraffes", "orangutans", "monkeys"];
    let trace1 = Bar::new(animals1, vec![20, 14, 23]).name("SF Zoo");

    let animals2 = vec!["giraffes", "orangutans", "monkeys"];
    let trace2 = Bar::new(animals2, vec![12, 18, 29]).name("LA Zoo");

    let layout = Layout::new().bar_mode(BarMode::Group);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

    plot.show();
}

fn stacked_bar_chart() {
    let animals1 = vec!["giraffes", "orangutans", "monkeys"];
    let trace1 = Bar::new(animals1, vec![20, 14, 23]).name("SF Zoo");

    let animals2 = vec!["giraffes", "orangutans", "monkeys"];
    let trace2 = Bar::new(animals2, vec![12, 18, 29]).name("LA Zoo");

    let layout = Layout::new().bar_mode(BarMode::Stack);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

    plot.show();
}

// Sankey Diagrams
fn basic_sankey_diagram() {
    // https://plotly.com/javascript/sankey-diagram/#basic-sankey-diagram
    let trace = Sankey::new()
        .orientation(Orientation::Horizontal)
        .node(
            Node::new()
                .pad(15)
                .thickness(30)
                .line(SankeyLine::new().color(NamedColor::Black).width(0.5))
                .label(vec!["A1", "A2", "B1", "B2", "C1", "C2"])
                .color_array(vec![
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                ]),
        )
        .link(
            Link::new()
                .value(vec![8, 4, 2, 8, 4, 2])
                .source(vec![0, 1, 0, 2, 3, 3])
                .target(vec![2, 3, 3, 4, 4, 5]),
        );

    let layout = Layout::new()
        .title("Basic Sankey".into())
        .font(Font::new().size(10));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);

    plot.show();
}

fn table_chart() {
    let trace = Table::new(
        Header::new(vec![String::from("col1"), String::from("col2")]),
        Cells::new(vec![vec![1, 2], vec![2, 3]]),
    );
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
}

fn main() {
    // Uncomment any of these lines to display the example.

    // Scatter Plots
    // simple_scatter_plot();
    // line_and_scatter_plots();
    // bubble_scatter_plots();
    // polar_scatter_plot();
    // data_labels_hover();
    // data_labels_on_the_plot();
    // colored_and_styled_scatter_plot();
    // large_data_sets();

    // Line Charts
    // adding_names_to_line_and_scatter_plot();
    // line_and_scatter_styling();
    // styling_line_plot();
    // line_shape_options_for_interpolation();
    // line_dash();
    // filled_lines();

    // Bar Charts
    // basic_bar_chart();
    // grouped_bar_chart();
    // stacked_bar_chart();
    // table_chart();

    // Sankey Diagrams
    // basic_sankey_diagram();
}
