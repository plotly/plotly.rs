use itertools_num::linspace;
use plotly::common::{Fill, Font, Mode};
use plotly::layout::{
    Axis, GridPattern, Layout, LayoutGrid, Margin, Shape, ShapeLayer, ShapeLine, ShapeType,
};
use plotly::{Bar, NamedColor, Plot, Scatter};
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

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

fn circles_positioned_relative_to_the_axes(show: bool) {
    let trace = Scatter::new(vec![1.5, 3.5], vec![0.75, 2.5])
        .text_array(vec!["Unfilled Circle", "Filled Circle"])
        .mode(Mode::Text);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 4.5]).zero_line(false))
        .y_axis(Axis::new().range(vec![0.0, 4.5]))
        .width(800)
        .height(800);

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(1)
            .y0(1)
            .x1(3)
            .y1(3)
            .line(ShapeLine::new().color(NamedColor::LightSeaGreen)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(3)
            .y0(3)
            .x1(4)
            .y1(4)
            .line(ShapeLine::new().color(NamedColor::LightSeaGreen))
            .fill_color(NamedColor::PaleTurquoise),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("circles_positioned_relative_to_the_axes"))
    );
}

fn highlighting_clusters_of_scatter_points_with_circle_shapes(show: bool) {
    let rng = thread_rng();
    let x0 = Normal::new(2., 0.45)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();
    let y0 = Normal::new(2., 0.45)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();
    let x1 = Normal::new(6., 0.4)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();
    let y1 = Normal::new(6., 0.4)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();
    let x2 = Normal::new(4., 0.3)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();
    let y2 = Normal::new(4., 0.3)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();

    let x0min = x0.iter().copied().fold(f64::NAN, f64::min);
    let x0max = x0.iter().copied().fold(f64::NAN, f64::max);
    let y0min = y0.iter().copied().fold(f64::NAN, f64::min);
    let y0max = y0.iter().copied().fold(f64::NAN, f64::max);

    let x1min = x1.iter().copied().fold(f64::NAN, f64::min);
    let x1max = x1.iter().copied().fold(f64::NAN, f64::max);
    let y1min = y1.iter().copied().fold(f64::NAN, f64::min);

    let x2min = x2.iter().copied().fold(f64::NAN, f64::min);
    let x2max = x2.iter().copied().fold(f64::NAN, f64::max);
    let y2min = y2.iter().copied().fold(f64::NAN, f64::min);

    let mut plot = Plot::new();
    plot.add_trace(Scatter::new(x0, y0.clone()).mode(Mode::Markers));
    plot.add_trace(Scatter::new(x1.clone(), y1).mode(Mode::Markers));
    plot.add_trace(Scatter::new(x2, y2).mode(Mode::Markers));
    plot.add_trace(Scatter::new(x1, y0).mode(Mode::Markers));

    let mut layout = Layout::new().show_legend(false);

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(x0min)
            .y0(y0min)
            .x1(x0max)
            .y1(y0max)
            .opacity(0.2)
            .fill_color(NamedColor::Blue)
            .line(ShapeLine::new().color(NamedColor::Blue)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(x1min)
            .y0(y1min)
            .x1(x1max)
            .y1(x1max)
            .opacity(0.2)
            .fill_color(NamedColor::Orange)
            .line(ShapeLine::new().color(NamedColor::Orange)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(x2min)
            .y0(y2min)
            .x1(x2max)
            .y1(x2max)
            .opacity(0.2)
            .fill_color(NamedColor::Green)
            .line(ShapeLine::new().color(NamedColor::Green)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(x1min)
            .y0(y0min)
            .x1(x1max)
            .y1(x0max)
            .opacity(0.2)
            .fill_color(NamedColor::Red)
            .line(ShapeLine::new().color(NamedColor::Red)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "highlighting_clusters_of_scatter_points_with_circle_shapes"
        ))
    );
}

fn venn_diagram_with_circle_shapes(show: bool) {
    let mut plot = Plot::new();
    plot.add_trace(
        Scatter::new(vec![1., 1.75, 2.5], vec![1., 1., 1.])
            .text_array(vec!["$A$", "$A+B$", "$B$"])
            .mode(Mode::Text)
            .text_font(
                Font::new()
                    .color(NamedColor::Black)
                    .size(18)
                    .family("Arial"),
            ),
    );

    let mut layout = Layout::new()
        .x_axis(
            Axis::new()
                .zero_line(false)
                .show_grid(false)
                .show_tick_labels(false),
        )
        .y_axis(
            Axis::new()
                .zero_line(false)
                .show_grid(false)
                .show_tick_labels(false),
        )
        .margin(Margin::new().left(20).right(20).bottom(100))
        .height(600)
        .width(800)
        .plot_background_color(NamedColor::White);

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(0)
            .y0(0)
            .x1(2)
            .y1(2)
            .opacity(0.3)
            .layer(ShapeLayer::Below)
            .fill_color(NamedColor::Blue)
            .line(ShapeLine::new().color(NamedColor::Blue)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(1.5)
            .y0(0.)
            .x1(3.5)
            .y1(2.)
            .opacity(0.3)
            .layer(ShapeLayer::Below)
            .fill_color(NamedColor::Gray)
            .line(ShapeLine::new().color(NamedColor::Gray)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("venn_diagram_with_circle_shapes"))
    );
}

fn adding_shapes_to_subplots(show: bool) {
    let mut plot = Plot::new();
    plot.add_trace(
        Scatter::new(vec![2, 6], vec![1, 1])
            .x_axis("x1")
            .y_axis("y1"),
    );
    plot.add_trace(
        Bar::new(vec![1, 2, 3], vec![4, 5, 6])
            .x_axis("x2")
            .y_axis("y2"),
    );
    plot.add_trace(
        Scatter::new(vec![10, 20], vec![40, 50])
            .x_axis("x3")
            .y_axis("y3"),
    );
    plot.add_trace(
        Bar::new(vec![11, 13, 15], vec![8, 11, 20])
            .x_axis("x4")
            .y_axis("y4"),
    );

    let mut layout = Layout::new()
        .grid(
            LayoutGrid::new()
                .rows(2)
                .columns(2)
                .pattern(GridPattern::Independent),
        )
        .x_axis(Axis::new().domain(&[0.0, 0.48]).anchor("x1"))
        .y_axis(Axis::new().domain(&[0.52, 1.]).anchor("y1"))
        .x_axis2(Axis::new().domain(&[0.52, 1.0]).anchor("x2"))
        .y_axis2(Axis::new().domain(&[0.5, 1.]).anchor("y2"))
        .x_axis3(Axis::new().domain(&[0.0, 0.48]).anchor("x3"))
        .y_axis3(Axis::new().domain(&[0.0, 0.48]).anchor("y3"))
        .x_axis4(Axis::new().domain(&[0.52, 1.0]).anchor("x4"))
        .y_axis4(Axis::new().domain(&[0.0, 0.48]).anchor("y4"));

    layout.add_shape(
        Shape::new()
            .x_ref("x1")
            .y_ref("y1")
            .shape_type(ShapeType::Line)
            .x0(3)
            .y0(0.5)
            .x1(5)
            .y1(0.8)
            .line(ShapeLine::new().width(3.)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x2")
            .y_ref("y2")
            .shape_type(ShapeType::Rect)
            .x0(4)
            .y0(2)
            .x1(5)
            .y1(6),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x3")
            .y_ref("y3")
            .shape_type(ShapeType::Rect)
            .x0(10)
            .y0(20)
            .x1(15)
            .y1(30),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x4")
            .y_ref("y4")
            .shape_type(ShapeType::Circle)
            .x0(5)
            .y0(12)
            .x1(10)
            .y1(18),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("adding_shapes_to_subplots")));
}

fn svg_paths(show: bool) {
    let mut plot = Plot::new();
    plot.add_trace(
        Scatter::new(vec![2, 1, 8, 8], vec![0.25, 9., 2., 6.])
            .text_array(vec![
                "Filled Triangle",
                "Filled Polygon",
                "Quadratic Bezier Curves",
                "Cubic Bezier Curves",
            ])
            .mode(Mode::Text),
    );

    let mut layout = Layout::new()
        .x_axis(
            Axis::new()
                .domain(&[0.05, 0.95])
                .range(vec![0., 9.])
                .zero_line(false),
        )
        .y_axis(
            Axis::new()
                .domain(&[0.05, 0.95])
                .range(vec![0, 11])
                .zero_line(false),
        );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Path)
            .path("M 4,4 Q 6,0 8,4")
            .line(ShapeLine::new().color(NamedColor::RoyalBlue)),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Path)
            .path("M 1,4 C 2,8 6,4 8,8")
            .line(ShapeLine::new().color(NamedColor::MediumPurple)),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Path)
            .path("M 1 1 L 1 3 L 4 1 Z")
            .fill_color(NamedColor::LightPink)
            .line(ShapeLine::new().color(NamedColor::Crimson)),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Path)
            .path("M 3,7 L2,8 L2,9 L3,10, L4,10 L5,9 L5,8 L4,7 Z")
            .fill_color(NamedColor::PaleTurquoise)
            .line(ShapeLine::new().color(NamedColor::LightSeaGreen)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("svg_paths")));
}

fn main() -> std::io::Result<()> {
    // Shapes
    // filled_area_chart(true);
    // vertical_and_horizontal_lines_positioned_relative_to_axes(true);
    // lines_positioned_relative_to_the_plot_and_to_the_axes(true);
    // creating_tangent_lines_with_shapes(true);
    // rectangles_positioned_relative_to_the_axes(true);
    // rectangle_positioned_relative_to_the_plot_and_to_the_axes(true);
    // highlighting_time_series_regions_with_rectangle_shapes(true);
    // circles_positioned_relative_to_the_axes(true);
    // highlighting_clusters_of_scatter_points_with_circle_shapes(true);
    // venn_diagram_with_circle_shapes(true);
    // adding_shapes_to_subplots(true);
    svg_paths(true);

    Ok(())
}
