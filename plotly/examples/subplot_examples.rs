use plotly::common::Title;
use plotly::layout::{Axis, GridPattern, Layout, LayoutGrid, Legend, RowOrder};
use plotly::{Plot, Scatter};

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
        .legend(Legend::new().trace_order("reversed"))
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

fn main() -> std::io::Result<()> {
    simple_subplot();
    custom_sized_subplot();
    multiple_subplots();
    stacked_subplots();
    stacked_subplots_with_shared_x_axis();
    multiple_custom_sized_subplots();
    Ok(())
}
