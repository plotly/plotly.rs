use plotly::common::{Font, Side, Title};
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Rgb, Scatter};

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
                .side(Side::Right),
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
                .side(Side::Left)
                .position(0.15),
        )
        .y_axis3(
            Axis::new()
                .title(Title::new("yaxis3 title").font(Font::new().color("#d62728")))
                .tick_font(Font::new().color("#d62728"))
                .anchor("x")
                .overlaying("y")
                .side(Side::Right),
        )
        .y_axis4(
            Axis::new()
                .title(Title::new("yaxis4 title").font(Font::new().color("#9467bd")))
                .tick_font(Font::new().color("#9467bd"))
                .anchor("free")
                .overlaying("y")
                .side(Side::Right)
                .position(0.85),
        );
    plot.set_layout(layout);

    plot.show();
}

fn main() -> std::io::Result<()> {
    two_y_axes();
    multiple_axes();
    Ok(())
}
