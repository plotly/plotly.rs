use std::error::Error;
use std::fs::File;

use minmaxlttb::{LttbBuilder, LttbMethod, Point};
use plotly::{common::DashType, Configuration, Layout, Plot, Scatter};
use plotly_utils::write_example_to_html;

const DATA_PATH: &str = "assets/timeseries.csv";

// ANCHOR: minmaxlttb_downsampling
fn minmaxlttb_downsampling(show: bool, file_name: &str) -> Result<(), Box<dyn Error>> {
    let original_data = load_timeseries_data()?;
    let ratio = 4;

    println!("Loaded timeseries data with {} points", original_data.len());

    let thresholds = vec![100, 250, 500, 1000, 2000];
    let mut downsampled_results = Vec::new();
    for &threshold in &thresholds {
        let downsampled = LttbBuilder::new()
            .threshold(threshold)
            .method(LttbMethod::MinMax)
            .build()
            .downsample(&original_data)
            .unwrap();
        println!(
            "MinMaxLttb (threshold = {threshold}, ratio = {ratio}): downsampled = {} points",
            downsampled.len()
        );
        downsampled_results.push((threshold, downsampled));
    }

    let mut plot = Plot::new();
    let x_orig: Vec<f64> = original_data.iter().map(|p| p.x()).collect();
    let y_orig: Vec<f64> = original_data.iter().map(|p| p.y()).collect();
    plot.add_trace(
        Scatter::new(x_orig, y_orig)
            .name(format!("Original Data ({})", original_data.len()))
            .line(
                plotly::common::Line::new()
                    .color("black")
                    .width(1.5)
                    .dash(DashType::Dash),
            ),
    );
    let colors = ["red", "blue", "green", "purple", "orange"];
    for (i, (threshold, downsampled)) in downsampled_results.iter().enumerate() {
        let x: Vec<f64> = downsampled.iter().map(|p| p.x()).collect();
        let y: Vec<f64> = downsampled.iter().map(|p| p.y()).collect();
        plot.add_trace(
            Scatter::new(x, y)
                .name(format!("MinMaxLttb ({threshold}"))
                .line(plotly::common::Line::new().color(colors[i]).width(1.5)),
        );
    }
    let layout = Layout::new()
        .title(plotly::common::Title::with_text(
            "MinMax LTTB Downsampling on Timeseries Data",
        ))
        .show_legend(true)
        .height(900)
        .x_axis(
            plotly::layout::Axis::new()
                .title(plotly::common::Title::with_text("Time"))
                .range(vec![0.0, original_data.last().unwrap().x()]),
        )
        .y_axis(
            plotly::layout::Axis::new()
                .title(plotly::common::Title::with_text("Value"))
                .range(vec![
                    original_data
                        .iter()
                        .map(|p| p.y())
                        .fold(f64::INFINITY, f64::min)
                        * 0.95,
                    original_data
                        .iter()
                        .map(|p| p.y())
                        .fold(f64::NEG_INFINITY, f64::max)
                        * 1.05,
                ]),
        );
    plot.set_layout(layout);
    plot.set_configuration(Configuration::default().responsive(true));

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }

    Ok(())
}
// ANCHOR_END: minmaxlttb_downsampling

fn load_timeseries_data() -> Result<Vec<Point>, Box<dyn Error>> {
    let file = File::open(DATA_PATH)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let mut data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let x: f64 = record.get(0).ok_or("Missing X column")?.parse()?;
        let y: f64 = record.get(1).ok_or("Missing Y column")?.parse()?;
        data.push(Point::new(x, y));
    }
    Ok(data)
}

fn main() {
    minmaxlttb_downsampling(false, "minmaxlttb_downsampling").unwrap();
}
