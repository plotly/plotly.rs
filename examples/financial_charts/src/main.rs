#![allow(dead_code)]

use std::env;
use std::path::PathBuf;

use chrono::{DateTime, Duration};
use plotly::common::TickFormatStop;
use plotly::layout::{Axis, RangeSelector, RangeSlider, SelectorButton, SelectorStep, StepMode};
use plotly::{Candlestick, Layout, Ohlc, Plot, Scatter};
use plotly_utils::write_example_to_html;
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
struct FinData {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    adjusted: f64,
    dn: f64,
    mavg: f64,
    up: f64,
    direction: String,
}

fn load_apple_data() -> Vec<FinData> {
    let mut p = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    p = p.join("assets").join("finance_charts_apple.csv");
    let mut rdr = csv::Reader::from_path(p).unwrap();
    let mut out = Vec::new();
    for result in rdr.deserialize() {
        let d: FinData = result.unwrap();
        out.push(d);
    }

    out
}

// Time Series and Date Axes
// ANCHOR: time_series_plot_with_custom_date_range
fn time_series_plot_with_custom_date_range(show: bool, file_name: &str) {
    let data = load_apple_data();
    let date: Vec<String> = data.iter().map(|d| d.date.clone()).collect();
    let high: Vec<f64> = data.iter().map(|d| d.high).collect();

    let trace = Scatter::new(date, high);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .x_axis(Axis::new().range(vec!["2016-07-01", "2016-12-31"]))
        .title("Manually Set Date Range");
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: time_series_plot_with_custom_date_range

// ANCHOR: time_series_with_range_slider
fn time_series_with_range_slider(show: bool, file_name: &str) {
    let data = load_apple_data();
    let date: Vec<String> = data.iter().map(|d| d.date.clone()).collect();
    let high: Vec<f64> = data.iter().map(|d| d.high).collect();

    let trace = Scatter::new(date, high);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .x_axis(Axis::new().range_slider(RangeSlider::new().visible(true)))
        .title("Manually Set Date Range");
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: time_series_with_range_slider

// ANCHOR: time_series_with_range_selector_buttons
fn time_series_with_range_selector_buttons(show: bool, file_name: &str) {
    let data = load_apple_data();
    let date: Vec<String> = data.iter().map(|d| d.date.clone()).collect();
    let high: Vec<f64> = data.iter().map(|d| d.high).collect();

    let trace = Scatter::new(date, high);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new().x_axis(
        Axis::new()
            .range_slider(RangeSlider::new().visible(true))
            .range_selector(RangeSelector::new().buttons(vec![
                        SelectorButton::new()
                            .count(1)
                            .label("1m")
                            .step(SelectorStep::Month)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(6)
                            .label("6m")
                            .step(SelectorStep::Month)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new()
                            .count(1)
                            .label("YTD")
                            .step(SelectorStep::Year)
                            .step_mode(StepMode::ToDate),
                        SelectorButton::new()
                            .count(1)
                            .label("1y")
                            .step(SelectorStep::Year)
                            .step_mode(StepMode::Backward),
                        SelectorButton::new().step(SelectorStep::All),
                    ])),
    );
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: time_series_with_range_selector_buttons

// ANCHOR: customizing_tick_label_formatting_by_zoom_level
fn customizing_tick_label_formatting_by_zoom_level(show: bool, file_name: &str) {
    let data = load_apple_data();
    let date: Vec<String> = data.iter().map(|d| d.date.clone()).collect();
    let high: Vec<f64> = data.iter().map(|d| d.high).collect();

    let trace = Scatter::new(date, high);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new().x_axis(
        Axis::new()
            .range_slider(RangeSlider::new().visible(true))
            .tick_format_stops(vec![
                TickFormatStop::new()
                    .dtick_range(vec![0, 1000])
                    .value("%H:%M:%S.%L ms"),
                TickFormatStop::new()
                    .dtick_range(vec![1000, 60000])
                    .value("%H:%M:%S s"),
                TickFormatStop::new()
                    .dtick_range(vec![60000, 3600000])
                    .value("%H:%M m"),
                TickFormatStop::new()
                    .dtick_range(vec![3600000, 86400000])
                    .value("%H:%M h"),
                TickFormatStop::new()
                    .dtick_range(vec![86400000, 604800000])
                    .value("%e. %b d"),
                TickFormatStop::new()
                    .dtick_range(vec!["M1", "M12"])
                    .value("%b '%y M"),
            ]),
    );
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: customizing_tick_label_formatting_by_zoom_level

// Candlestick Charts
// ANCHOR: simple_candlestick_chart
fn simple_candlestick_chart(show: bool, file_name: &str) {
    let x = vec![
        "2017-01-04",
        "2017-01-05",
        "2017-01-06",
        "2017-01-09",
        "2017-01-10",
        "2017-01-11",
        "2017-01-12",
        "2017-01-13",
        "2017-01-17",
        "2017-01-18",
        "2017-01-19",
        "2017-01-20",
        "2017-01-23",
        "2017-01-24",
        "2017-01-25",
        "2017-01-26",
        "2017-01-27",
        "2017-01-30",
        "2017-01-31",
        "2017-02-01",
        "2017-02-02",
        "2017-02-03",
        "2017-02-06",
        "2017-02-07",
        "2017-02-08",
        "2017-02-09",
        "2017-02-10",
        "2017-02-13",
        "2017-02-14",
        "2017-02-15",
    ];
    let open = vec![
        115.849998, 115.919998, 116.779999, 117.949997, 118.769997, 118.739998, 118.900002,
        119.110001, 118.339996, 120.0, 119.400002, 120.449997, 120.0, 119.550003, 120.419998,
        121.669998, 122.139999, 120.93, 121.150002, 127.029999, 127.980003, 128.309998, 129.130005,
        130.539993, 131.350006, 131.649994, 132.460007, 133.080002, 133.470001, 135.520004,
    ];
    let high = vec![
        116.510002, 116.860001, 118.160004, 119.43, 119.379997, 119.93, 119.300003, 119.620003,
        120.239998, 120.5, 120.089996, 120.449997, 120.809998, 120.099998, 122.099998, 122.440002,
        122.349998, 121.629997, 121.389999, 130.490005, 129.389999, 129.190002, 130.5, 132.089996,
        132.220001, 132.449997, 132.940002, 133.820007, 135.089996, 136.270004,
    ];
    let low = vec![
        115.75, 115.809998, 116.470001, 117.940002, 118.300003, 118.599998, 118.209999, 118.809998,
        118.220001, 119.709999, 119.370003, 119.730003, 119.769997, 119.5, 120.279999, 121.599998,
        121.599998, 120.660004, 120.620003, 127.010002, 127.779999, 128.160004, 128.899994,
        130.449997, 131.220001, 131.119995, 132.050003, 132.75, 133.25, 134.619995,
    ];
    let close = vec![
        116.019997, 116.610001, 117.910004, 118.989998, 119.110001, 119.75, 119.25, 119.040001,
        120.0, 119.989998, 119.779999, 120.0, 120.080002, 119.970001, 121.879997, 121.940002,
        121.949997, 121.629997, 121.349998, 128.75, 128.529999, 129.080002, 130.289993, 131.529999,
        132.039993, 132.419998, 132.119995, 133.289993, 135.020004, 135.509995,
    ];

    let trace1 = Candlestick::new(x, open, high, low, close);

    let mut plot = Plot::new();
    plot.add_trace(trace1);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: simple_candlestick_chart

// OHLC Charts
// ANCHOR: simple_ohlc_chart
fn simple_ohlc_chart(show: bool, file_name: &str) {
    let x = vec![
        "2017-01-04",
        "2017-01-05",
        "2017-01-06",
        "2017-01-09",
        "2017-01-10",
        "2017-01-11",
        "2017-01-12",
        "2017-01-13",
        "2017-01-17",
        "2017-01-18",
        "2017-01-19",
        "2017-01-20",
        "2017-01-23",
        "2017-01-24",
        "2017-01-25",
        "2017-01-26",
        "2017-01-27",
        "2017-01-30",
        "2017-01-31",
        "2017-02-01",
        "2017-02-02",
        "2017-02-03",
        "2017-02-06",
        "2017-02-07",
        "2017-02-08",
        "2017-02-09",
        "2017-02-10",
        "2017-02-13",
        "2017-02-14",
        "2017-02-15",
    ];
    let open = vec![
        115.849998, 115.919998, 116.779999, 117.949997, 118.769997, 118.739998, 118.900002,
        119.110001, 118.339996, 120.0, 119.400002, 120.449997, 120.0, 119.550003, 120.419998,
        121.669998, 122.139999, 120.93, 121.150002, 127.029999, 127.980003, 128.309998, 129.130005,
        130.539993, 131.350006, 131.649994, 132.460007, 133.080002, 133.470001, 135.520004,
    ];
    let high = vec![
        116.510002, 116.860001, 118.160004, 119.43, 119.379997, 119.93, 119.300003, 119.620003,
        120.239998, 120.5, 120.089996, 120.449997, 120.809998, 120.099998, 122.099998, 122.440002,
        122.349998, 121.629997, 121.389999, 130.490005, 129.389999, 129.190002, 130.5, 132.089996,
        132.220001, 132.449997, 132.940002, 133.820007, 135.089996, 136.270004,
    ];
    let low = vec![
        115.75, 115.809998, 116.470001, 117.940002, 118.300003, 118.599998, 118.209999, 118.809998,
        118.220001, 119.709999, 119.370003, 119.730003, 119.769997, 119.5, 120.279999, 121.599998,
        121.599998, 120.660004, 120.620003, 127.010002, 127.779999, 128.160004, 128.899994,
        130.449997, 131.220001, 131.119995, 132.050003, 132.75, 133.25, 134.619995,
    ];
    let close = vec![
        116.019997, 116.610001, 117.910004, 118.989998, 119.110001, 119.75, 119.25, 119.040001,
        120.0, 119.989998, 119.779999, 120.0, 120.080002, 119.970001, 121.879997, 121.940002,
        121.949997, 121.629997, 121.349998, 128.75, 128.529999, 129.080002, 130.289993, 131.529999,
        132.039993, 132.419998, 132.119995, 133.289993, 135.020004, 135.509995,
    ];

    let trace1 = Ohlc::new(x, open, high, low, close);

    let mut plot = Plot::new();
    plot.add_trace(trace1);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: simple_ohlc_chart

// ANCHOR: series_with_gaps_for_weekends_and_holidays
fn series_with_gaps_for_weekends_and_holidays(show: bool, file_name: &str) {
    let data = load_apple_data();

    // Filter data for the specific date range as in the Python example
    let filtered_data: Vec<&FinData> = data
        .iter()
        .filter(|d| d.date.as_str() >= "2015-12-01" && d.date.as_str() <= "2016-01-15")
        .collect();

    let date: Vec<String> = filtered_data.iter().map(|d| d.date.clone()).collect();
    let high: Vec<f64> = filtered_data.iter().map(|d| d.high).collect();

    let trace = Scatter::new(date, high).mode(plotly::common::Mode::Markers);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .title("Series with Weekend and Holiday Gaps")
        .x_axis(
            Axis::new()
                .range(vec!["2015-12-01", "2016-01-15"])
                .title("Date"),
        )
        .y_axis(Axis::new().title("Price"));
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: series_with_gaps_for_weekends_and_holidays

// ANCHOR: hiding_weekends_and_holidays_with_rangebreaks
fn hiding_weekends_and_holidays_with_rangebreaks(show: bool, file_name: &str) {
    let data = load_apple_data();

    // Filter data for the specific date range as in the Python example
    let filtered_data: Vec<&FinData> = data
        .iter()
        .filter(|d| d.date.as_str() >= "2015-12-01" && d.date.as_str() <= "2016-01-15")
        .collect();

    let date: Vec<String> = filtered_data.iter().map(|d| d.date.clone()).collect();
    let high: Vec<f64> = filtered_data.iter().map(|d| d.high).collect();

    let trace = Scatter::new(date, high).mode(plotly::common::Mode::Markers);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .title("Hide Weekend and Holiday Gaps with rangebreaks")
        .x_axis(
            Axis::new()
                .range(vec!["2015-12-01", "2016-01-15"])
                .title("Date")
                .range_breaks(vec![
                    plotly::layout::RangeBreak::new()
                        .bounds("sat", "mon"), // hide weekends
                    plotly::layout::RangeBreak::new()
                        .values(vec!["2015-12-25", "2016-01-01"]), // hide Christmas and New Year's
                ]),
        )
        .y_axis(Axis::new().title("Price"));
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: hiding_weekends_and_holidays_with_rangebreaks

// Helper to generate random walk data for all hours in a week
fn generate_business_hours_data() -> (Vec<String>, Vec<f64>) {
    use rand::Rng;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    let mut dates = Vec::new();
    let mut values = Vec::new();
    let mut current_value = 0.0;
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let start_date = DateTime::parse_from_rfc3339("2020-03-02T00:00:00Z").unwrap();
    for day in 0..5 {
        // Monday to Friday
        for hour in 0..24 {
            let current_date = start_date + Duration::days(day) + Duration::hours(hour);
            dates.push(current_date.format("%Y-%m-%d %H:%M:%S").to_string());
            current_value += (rng.gen::<f64>() - 0.5) * 2.0;
            values.push(current_value);
        }
    }
    (dates, values)
}

// ANCHOR: series_with_non_business_hours_gaps
fn series_with_non_business_hours_gaps(show: bool, file_name: &str) {
    use chrono::NaiveDateTime;
    use chrono::Timelike;
    let (dates, all_values) = generate_business_hours_data();
    let mut values = Vec::with_capacity(all_values.len());

    for (date_str, v) in dates.iter().zip(all_values.iter()) {
        // Parse the date string to extract hour
        // Format is "2020-03-02 09:00:00"
        if let Ok(datetime) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
            let hour = datetime.hour();
            if (9..17).contains(&hour) {
                values.push(*v);
            } else {
                values.push(f64::NAN);
            }
        } else {
            values.push(f64::NAN);
        }
    }

    let trace = Scatter::new(dates, values).mode(plotly::common::Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    let layout = Layout::new()
        .title("Series with Non-Business Hour Gaps")
        .x_axis(Axis::new().title("Time").tick_format("%b %d, %Y %H:%M"))
        .y_axis(Axis::new().title("Value"));
    plot.set_layout(layout);
    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: series_with_non_business_hours_gaps

// ANCHOR: hiding_non_business_hours_with_rangebreaks
fn hiding_non_business_hours_with_rangebreaks(show: bool, file_name: &str) {
    let (dates, values) = generate_business_hours_data();
    let trace = Scatter::new(dates, values).mode(plotly::common::Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    let layout = Layout::new()
        .title("Hide Non-Business Hour Gaps with rangebreaks")
        .x_axis(
            Axis::new()
                .title("Time")
                .tick_format("%b %d, %Y %H:%M")
                .range_breaks(vec![
                    plotly::layout::RangeBreak::new()
                        .bounds(17, 9)
                        .pattern("hour"), // hide hours outside of 9am-5pm
                ]),
        )
        .y_axis(Axis::new().title("Value"));
    plot.set_layout(layout);
    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: hiding_non_business_hours_with_rangebreaks

fn main() {
    // Change false to true on any of these lines to display the example.

    // Time Series and Date Axes

    time_series_plot_with_custom_date_range(false, "time_series_plot_with_custom_date_range");

    time_series_with_range_slider(false, "time_series_with_range_slider");

    time_series_with_range_selector_buttons(false, "time_series_with_range_selector_buttons");

    customizing_tick_label_formatting_by_zoom_level(
        false,
        "customizing_tick_label_formatting_by_zoom_level",
    );

    // Candlestick Charts
    simple_candlestick_chart(false, "simple_candlestick_chart");

    // OHLC Charts
    simple_ohlc_chart(false, "simple_ohlc_chart");

    // Rangebreaks usage
    series_with_gaps_for_weekends_and_holidays(false, "series_with_gaps_for_weekends_and_holidays");
    hiding_weekends_and_holidays_with_rangebreaks(
        false,
        "hiding_weekends_and_holidays_with_rangebreaks",
    );
    series_with_non_business_hours_gaps(false, "series_with_non_business_hours_gaps");
    hiding_non_business_hours_with_rangebreaks(false, "hiding_non_business_hours_with_rangebreaks");
}
