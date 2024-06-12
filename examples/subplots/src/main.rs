#![allow(dead_code)]

use plotly::common::{Anchor, AxisSide, Font, Title};
use plotly::layout::{
    Annotation, Axis, GridPattern, Layout, LayoutGrid, Legend, RowOrder, TraceOrder,
};
use plotly::Configuration;
use plotly::{color::Rgb, Plot, Scatter};
// Subplots
fn simple_subplot() {
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

    plot.show();
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

    plot.show();
}

fn multiple_subplots() {
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

    plot.show();
}

fn stacked_subplots() {
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

    plot.show();
}

fn stacked_subplots_with_shared_x_axis() {
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

    plot.show();
}

fn multiple_custom_sized_subplots() {
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
        .title(Title::new("Multiple Custom Sized Subplots"))
        .x_axis(Axis::new().domain(&[0., 0.45]).anchor("y1"))
        .y_axis(Axis::new().domain(&[0.5, 1.]).anchor("x1"))
        .x_axis2(Axis::new().domain(&[0.55, 1.]).anchor("y2"))
        .y_axis2(Axis::new().domain(&[0.8, 1.]).anchor("x2"))
        .x_axis3(Axis::new().domain(&[0.55, 1.]).anchor("y3"))
        .y_axis3(Axis::new().domain(&[0.5, 0.75]).anchor("x3"))
        .x_axis4(Axis::new().domain(&[0., 1.]).anchor("y4"))
        .y_axis4(Axis::new().domain(&[0., 0.45]).anchor("x4"));
    plot.set_layout(layout);

    plot.show();
}

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
        .title(Title::new("Double Y Axis Example"))
        .y_axis(Axis::new().title(Title::new("yaxis title")))
        .y_axis2(
            Axis::new()
                .title(Title::new("yaxis2 title").font(Font::new().color(Rgb::new(148, 103, 189))))
                .tick_font(Font::new().color(Rgb::new(148, 103, 189)))
                .overlaying("y")
                .side(AxisSide::Right),
        );
    plot.set_layout(layout);

    plot.show();
}

fn multiple_axes() {
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
        .title(Title::new("multiple y-axes example"))
        .width(800)
        .x_axis(Axis::new().domain(&[0.3, 0.7]))
        .y_axis(
            Axis::new()
                .title(Title::new("yaxis title").font(Font::new().color("#1f77b4")))
                .tick_font(Font::new().color("#1f77b4")),
        )
        .y_axis2(
            Axis::new()
                .title(Title::new("yaxis2 title").font(Font::new().color("#ff7f0e")))
                .tick_font(Font::new().color("#ff7f0e"))
                .anchor("free")
                .overlaying("y")
                .side(AxisSide::Left)
                .position(0.15),
        )
        .y_axis3(
            Axis::new()
                .title(Title::new("yaxis3 title").font(Font::new().color("#d62728")))
                .tick_font(Font::new().color("#d62728"))
                .anchor("x")
                .overlaying("y")
                .side(AxisSide::Right),
        )
        .y_axis4(
            Axis::new()
                .title(Title::new("yaxis4 title").font(Font::new().color("#9467bd")))
                .tick_font(Font::new().color("#9467bd"))
                .anchor("free")
                .overlaying("y")
                .side(AxisSide::Right)
                .position(0.85),
        );
    plot.set_layout(layout);

    plot.show();
}

fn many_subplots_with_titles() {
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
                .y_axis(format!("y{}", i))
                .x_axis(format!("x{}", i)),
        );
        layout.add_annotation(
            Annotation::new()
                .y_ref(format!("y{} domain", i))
                .y_anchor(Anchor::Bottom)
                .y(1)
                .text(format!("Title {}", i))
                .x_ref(format!("x{} domain", i))
                .x_anchor(Anchor::Center)
                .x(0.5)
                .show_arrow(false),
        )
    }

    plot.set_layout(layout);
    plot.set_configuration(Configuration::new().responsive(true));
    plot.show();
}

fn main() {
    // Uncomment any of these lines to display the example.

    // Subplots
    // simple_subplot();
    // custom_sized_subplot();
    // multiple_subplots();
    // stacked_subplots();
    // stacked_subplots_with_shared_x_axis();
    // multiple_custom_sized_subplots();
    // many_subplots_with_titles();

    // Multiple Axes
    // two_y_axes();
    // multiple_axes();
}
