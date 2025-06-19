#![allow(dead_code)]

use ndarray::Array;
use plotly::{
    color::{NamedColor, Rgb, Rgba},
    common::{
        ColorScale, ColorScalePalette, DashType, Domain, Fill, Font, HoverInfo, Line, LineShape,
        Marker, Mode, Orientation, Pattern, PatternShape,
    },
    layout::{
        Annotation, Axis, BarMode, CategoryOrder, Layout, LayoutGrid, Legend, TicksDirection,
        TraceOrder,
    },
    sankey::{Line as SankeyLine, Link, Node},
    traces::table::{Cells, Header},
    Bar, Pie, Plot, Sankey, Scatter, ScatterPolar, Table,
};
use plotly_utils::write_example_to_html;
use rand_distr::{Distribution, Normal, Uniform};

// Scatter Plots
// ANCHOR: simple_scatter_plot
fn simple_scatter_plot(show: bool, file_name: &str) {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec_and_offset().0;
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();

    let trace = Scatter::new(t, y).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: simple_scatter_plot

// ANCHOR: line_and_scatter_plots
fn line_and_scatter_plots(show: bool, file_name: &str) {
    let n: usize = 100;
    let mut rng = rand::rng();
    let random_x: Vec<f64> = Array::linspace(0., 1., n).into_raw_vec_and_offset().0;
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: line_and_scatter_plots

// ANCHOR: bubble_scatter_plots
fn bubble_scatter_plots(show: bool, file_name: &str) {
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: bubble_scatter_plots

fn polar_scatter_plot(show: bool, file_name: &str) {
    let n: usize = 400;
    let theta: Vec<f64> = Array::linspace(0., 360., n).into_raw_vec_and_offset().0;
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

// ANCHOR: data_labels_hover
fn data_labels_hover(show: bool, file_name: &str) {
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
        .title("Data Labels Hover")
        .x_axis(Axis::new().title("x").range(vec![0.75, 5.25]))
        .y_axis(Axis::new().title("y").range(vec![0., 8.]));
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: data_labels_hover

// ANCHOR: data_labels_on_the_plot
fn data_labels_on_the_plot(show: bool, file_name: &str) {
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
        .title("Data Labels on the Plot")
        .x_axis(Axis::new().range(vec![0.75, 5.25]))
        .y_axis(Axis::new().range(vec![0., 8.]));
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: data_labels_on_the_plot

// ANCHOR: colored_and_styled_scatter_plot
fn colored_and_styled_scatter_plot(show: bool, file_name: &str) {
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
        .title("Quarter 1 Growth")
        .x_axis(
            Axis::new()
                .title("GDP per Capita")
                .show_grid(false)
                .zero_line(false),
        )
        .y_axis(Axis::new().title("Percent").show_line(false));
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: colored_and_styled_scatter_plot

// ANCHOR: large_data_sets
fn large_data_sets(show: bool, file_name: &str) {
    let n: usize = 100_000;
    let mut rng = rand::rng();
    let r: Vec<f64> = Uniform::new(0., 1.)
        .unwrap()
        .sample_iter(&mut rng)
        .take(n)
        .collect();
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: large_data_sets

// Line Charts
// ANCHOR: adding_names_to_line_and_scatter_plot
fn adding_names_to_line_and_scatter_plot(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .mode(Mode::Markers)
        .name("Scatter");
    let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
        .mode(Mode::Lines)
        .name("Lines");
    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
        .mode(Mode::LinesMarkers)
        .name("Scatter + Lines");

    let layout = Layout::new().title("Adding Names to Line and Scatter Plot");
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: adding_names_to_line_and_scatter_plot

// ANCHOR: line_and_scatter_styling
fn line_and_scatter_styling(show: bool, file_name: &str) {
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

    let layout = Layout::new().title("Line and Scatter Styling");
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: line_and_scatter_styling

// ANCHOR: styling_line_plot
fn styling_line_plot(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .mode(Mode::Markers)
        .name("Red")
        .line(Line::new().color(Rgb::new(219, 64, 82)).width(3.0));
    let trace2 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
        .mode(Mode::LinesMarkers)
        .name("Blue")
        .line(Line::new().color(Rgb::new(55, 128, 191)).width(1.0));

    let layout = Layout::new()
        .title("Styling Line Plot")
        .width(500)
        .height(500);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: styling_line_plot

// ANCHOR: line_shape_options_for_interpolation
fn line_shape_options_for_interpolation(show: bool, file_name: &str) {
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: line_shape_options_for_interpolation

// ANCHOR: line_dash
fn line_dash(show: bool, file_name: &str) {
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: line_dash

// ANCHOR: filled_lines
fn filled_lines(show: bool, file_name: &str) {
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: filled_lines

/// Scatter plot showing y axis categories and category ordering.
// ANCHOR: categories_scatter_chart
fn categories_scatter_chart(show: bool, file_name: &str) {
    // Categories are ordered on the y axis from bottom to top.
    let categories = vec!["Unknown", "Off", "On"];

    let x = vec![
        "2024-10-30T08:30:05.05Z",
        "2024-10-30T08:35:05.05Z",
        "2024-10-30T08:50:05.05Z",
        "2024-10-30T08:50:20.05Z",
        "2024-10-30T09:00:05.05Z",
        "2024-10-30T09:05:05.05Z",
        "2024-10-30T09:10:05.05Z",
        "2024-10-30T09:10:20.05Z",
    ];
    let y = vec![
        "On",
        "Off",
        "Unknown",
        "Off",
        "On",
        "Off",
        // Categories that aren't in the category_array follow the Trace order.
        "NewCategory",
        "Off",
    ];

    let trace = Scatter::new(x, y).line(Line::new().shape(LineShape::Hv));

    let layout = Layout::new().y_axis(
        Axis::new()
            .category_order(CategoryOrder::Array)
            .category_array(categories),
    );

    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: categories_scatter_chart

// Bar Charts
// ANCHOR: basic_bar_chart
fn basic_bar_chart(show: bool, file_name: &str) {
    let animals = vec!["giraffes", "orangutans", "monkeys"];
    let t = Bar::new(animals, vec![20, 14, 23]);
    let mut plot = Plot::new();
    plot.add_trace(t);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: basic_bar_chart

// ANCHOR: grouped_bar_chart
fn grouped_bar_chart(show: bool, file_name: &str) {
    let animals1 = vec!["giraffes", "orangutans", "monkeys"];
    let trace1 = Bar::new(animals1, vec![20, 14, 23]).name("SF Zoo");

    let animals2 = vec!["giraffes", "orangutans", "monkeys"];
    let trace2 = Bar::new(animals2, vec![12, 18, 29]).name("LA Zoo");

    let layout = Layout::new().bar_mode(BarMode::Group);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: grouped_bar_chart

// ANCHOR: stacked_bar_chart
fn stacked_bar_chart(show: bool, file_name: &str) {
    let animals1 = vec!["giraffes", "orangutans", "monkeys"];
    let trace1 = Bar::new(animals1, vec![20, 14, 23]).name("SF Zoo");

    let animals2 = vec!["giraffes", "orangutans", "monkeys"];
    let trace2 = Bar::new(animals2, vec![12, 18, 29]).name("LA Zoo");

    let layout = Layout::new().bar_mode(BarMode::Stack);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: stacked_bar_chart

/// Graph a bar chart that orders the x axis categories by the total number
/// of animals in each category.
// ANCHOR: category_order_bar_chart
fn category_order_bar_chart(show: bool, file_name: &str) {
    let animals1 = vec!["giraffes", "orangutans", "monkeys"];
    let trace1 = Bar::new(animals1, vec![10, 14, 23]).name("SF Zoo");

    let animals2 = vec!["giraffes", "orangutans", "monkeys"];
    let trace2 = Bar::new(animals2, vec![12, 18, 29]).name("LA Zoo");

    let layout = Layout::new()
        .bar_mode(BarMode::Stack)
        // Order the x axis categories so the category with the most animals
        // appears first.
        .x_axis(Axis::new().category_order(CategoryOrder::TotalDescending));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: category_order_bar_chart

// ANCHOR: bar_chart_with_pattern_fills
fn bar_chart_with_pattern_fills(show: bool, file_name: &str) {
    let animals1 = vec!["giraffes", "orangutans", "monkeys"];
    let trace1 = Bar::new(animals1, vec![20, 14, 23]).name("SF Zoo").marker(
        Marker::new().line(Line::new().width(1.0)).pattern(
            Pattern::new()
                .shape(PatternShape::LeftDiagonalLine)
                .solidity(0.1),
        ),
    );

    let animals2 = vec!["giraffes", "orangutans", "monkeys"];
    let trace2 = Bar::new(animals2, vec![12, 18, 29]).name("LA Zoo").marker(
        Marker::new().line(Line::new().width(1.0)).pattern(
            Pattern::new()
                .shape(PatternShape::RightDiagonalLine)
                .solidity(0.5),
        ),
    );

    let layout = Layout::new().bar_mode(BarMode::Group);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: bar_chart_with_pattern_fills

// Sankey Diagrams
// ANCHOR: basic_sankey_diagram
fn basic_sankey_diagram(show: bool, file_name: &str) {
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
        .title("Basic Sankey")
        .font(Font::new().size(10));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: basic_sankey_diagram

// ANCHOR: custom_node_sankey_diagram
fn custom_node_sankey_diagram(show: bool, file_name: &str) {
    // https://plotly.com/javascript/sankey-diagram/#basic-sankey-diagram
    let trace = Sankey::new()
        .orientation(Orientation::Horizontal)
        .arrangement(plotly::sankey::Arrangement::Snap)
        .node(
            Node::new()
                .pad(15)
                .thickness(30)
                .line(SankeyLine::new().color(NamedColor::Black).width(0.5))
                .label(vec!["A", "B", "C", "D", "E", "F"])
                .x(vec![0.2, 0.1, 0.5, 0.7, 0.3, 0.5])
                .y(vec![0.2, 0.1, 0.5, 0.7, 0.3, 0.5])
                .pad(20),
        )
        .link(
            Link::new()
                .source(vec![0, 0, 1, 2, 5, 4, 3, 5])
                .target(vec![5, 3, 4, 3, 0, 2, 2, 3])
                .value(vec![1, 2, 1, 1, 1, 1, 1, 2]),
        );

    let layout = Layout::new()
        .title("Define Node Position")
        .font(Font::new().size(10));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: custom_node_sankey_diagram

// ANCHOR: table_chart
fn table_chart(show: bool, file_name: &str) {
    let trace = Table::new(
        Header::new(vec![String::from("col1"), String::from("col2")]),
        Cells::new(vec![vec![1, 2], vec![2, 3]]),
    );
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: table_chart

// Pie Charts
// ANCHOR: basic_pie_chart
fn basic_pie_chart(show: bool, file_name: &str) {
    let values = vec![2, 3, 4];
    let labels = vec!["giraffes", "orangutans", "monkeys"];
    let t = Pie::new(values).labels(labels);
    let mut plot = Plot::new();
    plot.add_trace(t);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: basic_pie_chart

// ANCHOR: basic_pie_chart_labels
fn basic_pie_chart_labels(show: bool, file_name: &str) {
    let labels = ["giraffes", "giraffes", "orangutans", "monkeys"];
    let t = Pie::<u32>::from_labels(&labels);
    let mut plot = Plot::new();
    plot.add_trace(t);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: basic_pie_chart_labels

// ANCHOR: pie_chart_text_control
fn pie_chart_text_control(show: bool, file_name: &str) {
    let values = vec![2, 3, 4, 4];
    let labels = vec!["Wages", "Operating expenses", "Cost of sales", "Insurance"];
    let t = Pie::new(values)
        .labels(labels)
        .automargin(true)
        .show_legend(true)
        .text_position(plotly::common::Position::Outside)
        .name("Costs")
        .text_info("label+percent");
    let mut plot = Plot::new();
    plot.add_trace(t);

    let layout = Layout::new().height(700).width(700).show_legend(true);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: pie_chart_text_control

// ANCHOR: grouped_donout_pie_charts
fn grouped_donout_pie_charts(show: bool, file_name: &str) {
    let mut plot = Plot::new();

    let values = vec![16, 15, 12, 6, 5, 4, 42];
    let labels = vec![
        "US",
        "China",
        "European Union",
        "Russian Federation",
        "Brazil",
        "India",
        "Rest of World",
    ];
    let t = Pie::new(values)
        .labels(labels)
        .name("GHG Emissions")
        .hover_info(HoverInfo::All)
        .text("GHG")
        .hole(0.4)
        .domain(Domain::new().column(0));
    plot.add_trace(t);

    let values = vec![27, 11, 25, 8, 1, 3, 25];
    let labels = vec![
        "US",
        "China",
        "European Union",
        "Russian Federation",
        "Brazil",
        "India",
        "Rest of World",
    ];

    let t = Pie::new(values)
        .labels(labels)
        .name("CO2 Emissions")
        .hover_info(HoverInfo::All)
        .text("CO2")
        .text_position(plotly::common::Position::Inside)
        .hole(0.4)
        .domain(Domain::new().column(1));
    plot.add_trace(t);

    let layout = Layout::new()
        .title("Global Emissions 1990-2011")
        .height(400)
        .width(600)
        .annotations(vec![
            Annotation::new()
                .font(Font::new().size(20))
                .show_arrow(false)
                .text("GHG")
                .x(0.17)
                .y(0.5),
            Annotation::new()
                .font(Font::new().size(20))
                .show_arrow(false)
                .text("CO2")
                .x(0.82)
                .y(0.5),
        ])
        .show_legend(false)
        .grid(
            LayoutGrid::new()
                .columns(2)
                .rows(1)
                .pattern(plotly::layout::GridPattern::Independent),
        );
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: grouped_donout_pie_charts

fn main() {
    // Change false to true on any of these lines to display the example.

    // Scatter Plots
    simple_scatter_plot(false, "simple_scatter_plot");
    line_and_scatter_plots(false, "line_and_scatter_plots");
    bubble_scatter_plots(false, "bubble_scatter_plots");
    polar_scatter_plot(false, "polar_scatter_plot");
    data_labels_hover(false, "data_labels_hover");
    data_labels_on_the_plot(false, "data_labels_on_the_plot");

    colored_and_styled_scatter_plot(false, "colored_and_styled_scatter_plot");
    large_data_sets(false, "large_data_sets");
    categories_scatter_chart(false, "categories_scatter_chart");

    // Line Charts

    adding_names_to_line_and_scatter_plot(false, "adding_names_to_line_and_scatter_plot");
    line_and_scatter_styling(false, "line_and_scatter_styling");
    styling_line_plot(false, "styling_line_plot");

    line_shape_options_for_interpolation(false, "line_shape_options_for_interpolation");
    line_dash(false, "line_dash");
    filled_lines(false, "filled_lines");

    // Bar Charts
    basic_bar_chart(false, "basic_bar_chart");
    grouped_bar_chart(false, "grouped_bar_chart");
    stacked_bar_chart(false, "stacked_bar_chart");
    table_chart(false, "table_chart");
    category_order_bar_chart(false, "category_order_bar_chart");

    bar_chart_with_pattern_fills(false, "bar_chart_with_pattern_fills");

    // Sankey Diagrams
    basic_sankey_diagram(false, "basic_sankey_diagram");
    custom_node_sankey_diagram(false, "custom_node_sankey_diagram");

    // Pie Charts
    basic_pie_chart(false, "basic_pie_chart");
    basic_pie_chart_labels(false, "basic_pie_chart_labels");
    pie_chart_text_control(false, "pie_chart_text_control");

    grouped_donout_pie_charts(false, "grouped_donout_pie_charts");
}
