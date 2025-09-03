#![allow(dead_code)]

use plotly::common::{Anchor, AxisSide, Font, Title};
use plotly::layout::{
    Annotation, Axis, GridPattern, Layout, LayoutGrid, Legend, RowOrder, TraceOrder,
};
use plotly::Configuration;
use plotly::{color::Rgb, Plot, Scatter};
use plotly_utils::write_example_to_html;

// Subplots
// ANCHOR: simple_subplot
fn simple_subplot(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![20, 30, 40], vec![50, 60, 70])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(1)
            .columns(2)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: simple_subplot

// ANCHOR: simple_subplot_matches_x_axis
fn simple_subplot_matches_x_axis(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![20, 30, 40], vec![50, 60, 70])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().x_axis(Axis::new().matches("x2")).grid(
        LayoutGrid::new()
            .rows(1)
            .columns(2)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

fn simple_subplot_matches_y_axis() {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![20, 30, 40], vec![50, 60, 70])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().y_axis(Axis::new().matches("y2")).grid(
        LayoutGrid::new()
            .rows(1)
            .columns(2)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

fn custom_sized_subplot() {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![20, 30, 40], vec![50, 60, 70])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .x_axis(Axis::new().domain(&[0., 0.7]))
        .y_axis2(Axis::new().anchor("x2"))
        .x_axis2(Axis::new().domain(&[0.8, 1.]));
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: custom_sized_subplot

// ANCHOR: multiple_subplots
fn multiple_subplots(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![20, 30, 40], vec![50, 60, 70])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");
    let trace3 = Scatter::new(vec![300, 400, 500], vec![600, 700, 800])
        .x_axis("x3")
        .y_axis("y3");
    let trace4 = Scatter::new(vec![4000, 5000, 6000], vec![7000, 8000, 9000])
        .x_axis("x4")
        .y_axis("y4");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(2)
            .columns(2)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: multiple_subplots

// ANCHOR: stacked_subplots
fn stacked_subplots(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![0, 1, 2], vec![10, 11, 12]).name("trace1");
    let trace2 = Scatter::new(vec![2, 3, 4], vec![100, 110, 120])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");
    let trace3 = Scatter::new(vec![3, 4, 5], vec![1000, 1100, 1200])
        .x_axis("x3")
        .y_axis("y3");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(3)
            .columns(1)
            .pattern(GridPattern::Independent)
            .row_order(RowOrder::BottomToTop),
    );
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: stacked_subplots

// ANCHOR: stacked_subplots_with_shared_x_axis
fn stacked_subplots_with_shared_x_axis(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![0, 1, 2], vec![10, 11, 12]).name("trace1");
    let trace2 = Scatter::new(vec![2, 3, 4], vec![100, 110, 120])
        .name("trace2")
        .y_axis("y2");
    let trace3 = Scatter::new(vec![3, 4, 5], vec![1000, 1100, 1200]).y_axis("y3");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    let layout = Layout::new()
        .y_axis(Axis::new().domain(&[0., 0.33]))
        .legend(Legend::new().trace_order(TraceOrder::Reversed))
        .y_axis2(Axis::new().domain(&[0.33, 0.66]))
        .y_axis3(Axis::new().domain(&[0.66, 1.]));
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: stacked_subplots_with_shared_x_axis

// ANCHOR: multiple_custom_sized_subplots
fn multiple_custom_sized_subplots(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![1, 2], vec![1, 2]).name("(1,1)");
    let trace2 = Scatter::new(vec![1, 2], vec![1, 2])
        .name("(1,2,1)")
        .x_axis("x2")
        .y_axis("y2");
    let trace3 = Scatter::new(vec![1, 2], vec![1, 2])
        .name("(1,2,2)")
        .x_axis("x3")
        .y_axis("y3");
    let trace4 = Scatter::new(vec![1, 2], vec![1, 2])
        .name("{(2,1), (2,2)}")
        .x_axis("x4")
        .y_axis("y4");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);

    let layout = Layout::new()
        .title("Multiple Custom Sized Subplots")
        .x_axis(Axis::new().domain(&[0., 0.45]).anchor("y1"))
        .y_axis(Axis::new().domain(&[0.5, 1.]).anchor("x1"))
        .x_axis2(Axis::new().domain(&[0.55, 1.]).anchor("y2"))
        .y_axis2(Axis::new().domain(&[0.8, 1.]).anchor("x2"))
        .x_axis3(Axis::new().domain(&[0.55, 1.]).anchor("y3"))
        .y_axis3(Axis::new().domain(&[0.5, 0.75]).anchor("x3"))
        .x_axis4(Axis::new().domain(&[0., 1.]).anchor("y4"))
        .y_axis4(Axis::new().domain(&[0., 0.45]).anchor("x4"));
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: multiple_custom_sized_subplots

// Multiple Axes
fn two_y_axes() {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![40, 50, 60]).name("trace1");
    let trace2 = Scatter::new(vec![2, 3, 4], vec![4, 5, 6])
        .name("trace2")
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .title("Double Y Axis Example")
        .y_axis(Axis::new().title("yaxis title"))
        .y_axis2(
            Axis::new()
                .title(Title::from("yaxis2 title").font(Font::new().color(Rgb::new(148, 103, 189))))
                .tick_font(Font::new().color(Rgb::new(148, 103, 189)))
                .overlaying("y")
                .side(AxisSide::Right),
        );
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: two_y_axes

// ANCHOR: multiple_axes
fn multiple_axes(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![2, 3, 4], vec![40, 50, 60])
        .name("trace2")
        .y_axis("y2");
    let trace3 = Scatter::new(vec![4, 5, 6], vec![40_000, 50_000, 60_000]).y_axis("y3");
    let trace4 = Scatter::new(vec![5, 6, 7], vec![400_000, 500_000, 600_000]).y_axis("y4");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);

    let layout = Layout::new()
        .title("multiple y-axes example")
        .width(800)
        .x_axis(Axis::new().domain(&[0.3, 0.7]))
        .y_axis(
            Axis::new()
                .title(Title::from("yaxis title").font(Font::new().color("#1f77b4")))
                .tick_font(Font::new().color("#1f77b4")),
        )
        .y_axis2(
            Axis::new()
                .title(Title::from("yaxis2 title").font(Font::new().color("#ff7f0e")))
                .tick_font(Font::new().color("#ff7f0e"))
                .anchor("free")
                .overlaying("y")
                .side(AxisSide::Left)
                .position(0.15),
        )
        .y_axis3(
            Axis::new()
                .title(Title::from("yaxis3 title").font(Font::new().color("#d62728")))
                .tick_font(Font::new().color("#d62728"))
                .anchor("x")
                .overlaying("y")
                .side(AxisSide::Right),
        )
        .y_axis4(
            Axis::new()
                .title(Title::from("yaxis4 title").font(Font::new().color("#9467bd")))
                .tick_font(Font::new().color("#9467bd"))
                .anchor("free")
                .overlaying("y")
                .side(AxisSide::Right)
                .position(0.85),
        );
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: multiple_axes

// ANCHOR: many_subplots_with_titles
fn many_subplots_with_titles(show: bool, file_name: &str) {
    let trace1 = Scatter::new(vec![1, 2], vec![4, 5]);

    let number_of_plots = 10;

    let mut plot = Plot::new();
    let mut layout = Layout::new()
        .grid(
            LayoutGrid::new()
                .rows(number_of_plots / 2)
                .columns(2)
                .pattern(GridPattern::Independent),
        )
        .height(number_of_plots * 200);

    for i in 1..number_of_plots + 1 {
        plot.add_trace(
            trace1
                .clone()
                .y_axis(format!("y{i}"))
                .x_axis(format!("x{i}")),
        );
        layout.add_annotation(
            Annotation::new()
                .y_ref(format!("y{i} domain"))
                .y_anchor(Anchor::Bottom)
                .y(1)
                .text(format!("Title {i}"))
                .x_ref(format!("x{i} domain"))
                .x_anchor(Anchor::Center)
                .x(0.5)
                .show_arrow(false),
        )
    }

    plot.set_layout(layout);
    plot.set_configuration(Configuration::new().responsive(true));

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: many_subplots_with_titles

// ANCHOR: subplots_with_multiple_traces
fn subplots_with_multiple_traces(show: bool, file_name: &str) {
    // Create multiple traces for the first subplot (left side)
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 11, 12, 13])
        .name("Line 1")
        .mode(plotly::common::Mode::LinesMarkers);

    let trace2 = Scatter::new(vec![1, 2, 3, 4], vec![8, 9, 10, 11])
        .name("Line 2")
        .mode(plotly::common::Mode::LinesMarkers);

    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 13, 14, 15])
        .name("Line 3")
        .mode(plotly::common::Mode::LinesMarkers);

    // Create traces for the second subplot (right side)
    let trace4 = Scatter::new(vec![1, 2, 3, 4], vec![20, 25, 30, 35])
        .name("Dots 1")
        .x_axis("x2")
        .y_axis("y2")
        .mode(plotly::common::Mode::Markers);

    let trace5 = Scatter::new(vec![1, 2, 3, 4], vec![15, 20, 25, 30])
        .name("Dots 2")
        .x_axis("x2")
        .y_axis("y2")
        .mode(plotly::common::Mode::Markers);

    let mut plot = Plot::new();
    // Add traces to first subplot (default axes)
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    // Add traces to second subplot (x2, y2 axes)
    plot.add_trace(trace4);
    plot.add_trace(trace5);

    let layout = Layout::new().title("Subplots with Multiple Traces").grid(
        LayoutGrid::new()
            .rows(1)
            .columns(2)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: subplots_with_multiple_traces

fn main() {
    // Change false to true on any of these lines to display the example.
    // Subplots
    simple_subplot(false, "simple_subplot");

    simple_subplot_matches_x_axis(false, "simple_subplot_matches_x_axis");

    simple_subplot_matches_y_axis(false, "simple_subplot_matches_y_axis");
    custom_sized_subplot(false, "custom_sized_subplot");
    multiple_subplots(false, "multiple_subplots");
    stacked_subplots(false, "stacked_subplots");

    stacked_subplots_with_shared_x_axis(false, "stacked_subplots_with_shared_x_axis");

    multiple_custom_sized_subplots(false, "multiple_custom_sized_subplots");

    many_subplots_with_titles(false, "many_subplots_with_titles");

    // Multiple traces in subplots
    subplots_with_multiple_traces(false, "subplots_with_multiple_traces");

    // Multiple Axes
    two_y_axes(false, "two_y_axes");
    multiple_axes(false, "multiple_axes");
}
