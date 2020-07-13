use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, PlotType,
    Title,
};
use plotly::layout::{
    Axis, BarMode, Layout, Legend, Shape, ShapeLayer, ShapeLine, ShapeType, TicksDirection,
};
use plotly::{Bar, NamedColor, Plot, Rgb, Rgba, Scatter};
use rand_distr::{Distribution, Normal, Uniform};

// Shapes
fn filled_area_chart(show: bool) {
    let trace1 = Scatter::new(vec![0, 1, 2, 0], vec![0, 2, 0, 0]).fill(Fill::ToSelf);
    let trace2 =
        Scatter::new(vec![3, 3, 5, 5, 3], vec![0.5, 1.5, 1.5, 0.5, 0.5]).fill(Fill::ToSelf);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("filled_area_chart")));
}

fn vertical_and_horizontal_lines_positioned_relative_to_axes(show: bool) {
    let trace = Scatter::new(vec![2.0, 3.5, 6.0], vec![1.0, 1.5, 1.0])
        .text_array(vec![
            "Vertical Line",
            "Horizontal Dashed Line",
            "Diagonal dotted Line",
        ])
        .mode(Mode::Text);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 7.0]))
        .y_axis(Axis::new().range(vec![0.0, 2.5]));

    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(1)
            .y0(0)
            .x1(1)
            .y1(2)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue).width(3.)),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(2)
            .y0(2)
            .x1(5)
            .y1(2)
            .line(
                ShapeLine::new()
                    .color(NamedColor::LightSeaGreen)
                    .width(3.)
                    .dash("dashdot"),
            ),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(4)
            .y0(0)
            .x1(6)
            .y1(2)
            .line(
                ShapeLine::new()
                    .color(NamedColor::MediumPurple)
                    .width(3.)
                    .dash("dot"),
            ),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "vertical_and_horizontal_lines_positioned_relative_to_axes"
        ))
    );
}

fn lines_positioned_relative_to_the_plot_and_to_the_axes(show: bool) {
    let trace = Scatter::new(vec![2.0, 6.0], vec![1.0, 1.0])
        .text_array(vec![
            "Line positioned relative to the plot",
            "Line positioned relative to the axes",
        ])
        .mode(Mode::Text);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 8.0]))
        .y_axis(Axis::new().range(vec![0.0, 2.]));

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Line)
            .x0(4)
            .y0(0)
            .x1(8)
            .y1(1)
            .line(ShapeLine::new().color(NamedColor::LightSeaGreen).width(3.)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("paper")
            .y_ref("paper")
            .shape_type(ShapeType::Line)
            .x0(0.0)
            .y0(0.0)
            .x1(0.5)
            .y1(0.5)
            .line(ShapeLine::new().color(NamedColor::DarkOrange).width(3.)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "lines_positioned_relative_to_the_plot_and_to_the_axes"
        ))
    );
}

fn creating_tangent_lines_with_shapes(show: bool) {
    let x0: Vec<f64> = linspace(1.0, 3.0, 200).collect();
    let y0: Vec<f64> = x0.iter().map(|v| *v * (v.powf(2.)).sin() + 1.).collect();

    let trace = Scatter::new(x0, y0);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout =
        Layout::new().title("$f(x)=x\\sin(x^2)+1\\\\ f\'(x)=\\sin(x^2)+2x^2\\cos(x^2)$".into());

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .opacity(0.7)
            .shape_type(ShapeType::Line)
            .x0(1.)
            .y0(2.30756)
            .x1(1.75)
            .y1(2.30756)
            .line(ShapeLine::new().color(NamedColor::Crimson).width(2.5)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .opacity(0.7)
            .shape_type(ShapeType::Line)
            .x0(2.5)
            .y0(3.80796)
            .x1(3.05)
            .y1(3.80796)
            .line(ShapeLine::new().color(NamedColor::Crimson).width(2.5)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .opacity(0.7)
            .shape_type(ShapeType::Line)
            .x0(1.90)
            .y0(-1.1827)
            .x1(2.5)
            .y1(-1.1827)
            .line(ShapeLine::new().color(NamedColor::Crimson).width(2.5)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("creating_tangent_lines_with_shapes"))
    );
}

fn rectangles_positioned_relative_to_the_axes(show: bool) {
    let trace = Scatter::new(vec![1.5, 4.5], vec![0.75, 0.75])
        .text_array(vec!["Unfilled Rectangle", "Filled Rectangle"])
        .mode(Mode::Text);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 7.0]).show_grid(false))
        .y_axis(Axis::new().range(vec![0.0, 3.5]));

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Rect)
            .x0(1.)
            .y0(1.)
            .x1(2.)
            .y1(3.)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Rect)
            .x0(3.)
            .y0(1.)
            .x1(6.)
            .y1(2.)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue).width(2.))
            .fill_color(NamedColor::LightSkyBlue),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("rectangles_positioned_relative_to_the_axes"))
    );
}

fn rectangle_positioned_relative_to_the_plot_and_to_the_axes(show: bool) {
    let trace = Scatter::new(vec![1.5, 3.], vec![2.5, 2.5])
        .text_array(vec![
            "Rectangle reference to the plot",
            "Rectangle reference to the axes",
        ])
        .mode(Mode::Text);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 4.0]).show_grid(false))
        .y_axis(Axis::new().range(vec![0.0, 4.0]));

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Rect)
            .x0(2.5)
            .y0(0.0)
            .x1(3.5)
            .y1(2.0)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue).width(3.))
            .fill_color(NamedColor::LightSkyBlue),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("paper")
            .y_ref("paper")
            .shape_type(ShapeType::Rect)
            .x0(0.25)
            .y0(0.0)
            .x1(0.5)
            .y1(0.5)
            .line(ShapeLine::new().color(NamedColor::LightSeaGreen).width(3.))
            .fill_color(NamedColor::PaleTurquoise),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "rectangle_positioned_relative_to_the_plot_and_to_the_axes"
        ))
    );
}

fn highlighting_time_series_regions_with_rectangle_shapes(show: bool) {
    let x = vec![
        "2015-02-01",
        "2015-02-02",
        "2015-02-03",
        "2015-02-04",
        "2015-02-05",
        "2015-02-06",
        "2015-02-07",
        "2015-02-08",
        "2015-02-09",
        "2015-02-10",
        "2015-02-11",
        "2015-02-12",
        "2015-02-13",
        "2015-02-14",
        "2015-02-15",
        "2015-02-16",
        "2015-02-17",
        "2015-02-18",
        "2015-02-19",
        "2015-02-20",
        "2015-02-21",
        "2015-02-22",
        "2015-02-23",
        "2015-02-24",
        "2015-02-25",
        "2015-02-26",
        "2015-02-27",
        "2015-02-28",
    ];
    let y = vec![
        -14, -17, -8, -4, -7, -10, -12, -14, -12, -7, -11, -7, -18, -14, -14, -16, -13, -7, -8,
        -14, -8, -3, -9, -9, -4, -13, -9, -6,
    ];
    let trace = Scatter::new(x, y).mode(Mode::Lines).name("temperature");
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new();

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("paper")
            .shape_type(ShapeType::Rect)
            .x0("2015-02-04")
            .y0(0)
            .x1("2015-02-06")
            .y1(1)
            .fill_color(NamedColor::LightSalmon)
            .opacity(0.5)
            .layer(ShapeLayer::Below)
            .line(ShapeLine::new().width(0.)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("paper")
            .shape_type(ShapeType::Rect)
            .x0("2015-02-20")
            .y0(0)
            .x1("2015-02-22")
            .y1(1)
            .fill_color(NamedColor::LightSalmon)
            .opacity(0.5)
            .layer(ShapeLayer::Below)
            .line(ShapeLine::new().width(0.)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "highlighting_time_series_regions_with_rectangle_shapes"
        ))
    );
}

fn circles_positioned_relative_to_the_axes(show: bool) {}

fn highlighting_clusters_of_scatter_points_with_circle_shapes(show: bool) {}

fn venn_diagram_with_circle_shapes(show: bool) {}

fn adding_shapes_to_subplots(show: bool) {}

fn svg_paths(show: bool) {}

fn main() -> std::io::Result<()> {
    // Shapes
    filled_area_chart(true);
    vertical_and_horizontal_lines_positioned_relative_to_axes(true);
    lines_positioned_relative_to_the_plot_and_to_the_axes(true);
    creating_tangent_lines_with_shapes(true);
    rectangles_positioned_relative_to_the_axes(true);
    rectangle_positioned_relative_to_the_plot_and_to_the_axes(true);

    highlighting_time_series_regions_with_rectangle_shapes(true);
    circles_positioned_relative_to_the_axes(true);
    highlighting_clusters_of_scatter_points_with_circle_shapes(true);
    venn_diagram_with_circle_shapes(true);
    adding_shapes_to_subplots(true);
    svg_paths(true);

    Ok(())
}
