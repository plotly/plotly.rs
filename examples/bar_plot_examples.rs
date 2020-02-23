use plotly::charts::Layout;
use plotly::charts::{Bar, BarMode, ErrorData, ErrorType};
use plotly::Plot;

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
    plot.add_layout(layout);
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
    plot.add_layout(layout);
    plot.show();
}

fn bar_chart_with_error_bars() {
    let trace1 = Bar::new(
        vec![
            "Trial 1".to_owned(),
            "Trial 2".to_owned(),
            "Trial 3".to_owned(),
        ],
        vec![3, 6, 4],
    )
    .name("Control")
    .error_y(ErrorData::new(ErrorType::Data).array(vec![1.0, 0.5, 1.5]));

    let trace2 = Bar::new(
        vec![
            "Trial 1".to_owned(),
            "Trial 2".to_owned(),
            "Trial 3".to_owned(),
        ],
        vec![4, 7, 3],
    )
    .name("LA Zoo")
    .error_y(ErrorData::new(ErrorType::Data).array(vec![0.5, 1.0, 2.0]));

    let layout = Layout::new();
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_layout(layout);
    plot.show();
    plot.show_png(1024, 680);
}

fn main() -> std::io::Result<()> {
    basic_bar_chart();
    grouped_bar_chart();
    stacked_bar_chart();
    bar_chart_with_error_bars();
    Ok(())
}
