use chrono::prelude::*;
use chrono::{Date, DateTime, Duration, TimeZone, Utc};
use plotly::common::Mode;
use plotly::{Plot, Scatter};
use plotly_ndarray::{Array, ArrayTraces, Ix1, Ix2};
use std::ops::Add;

fn simple_scatter_plot() {
    let n: usize = 11;
    let t: Array<f64, Ix1> = Array::range(0., 10., 10. / n as f64);
    let mut ys: Array<f64, Ix2> = Array::zeros((11, 11));
    let mut count = 0.;
    for mut row in ys.gencolumns_mut() {
        for index in 0..row.len() {
            row[index] = count + (index as f64).powf(2.);
        }
        count += 1.;
    }

    let traces =
        Scatter::default()
            .mode(Mode::LinesMarkers)
            .to_traces(t, ys, ArrayTraces::OverColumns);

    let mut plot = Plot::new();
    plot.add_traces(traces);
    plot.show();
}

fn chrono_plot() {
    let times = vec![Utc::now(), Utc::now()];
    let y = vec![1., 10.];
    let trace = Scatter::time_series(times, y);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
}

fn date_range<Tz: TimeZone>(
    start_date: DateTime<Tz>,
    end_date: DateTime<Tz>,
    n: usize,
) -> Vec<DateTime<Tz>> {
    if n < 2 {
        panic!("date_range: n must be greater or equal to 2");
    }
    let mut result: Vec<DateTime<Tz>> = Vec::new();
    let total_duration = end_date
        .signed_duration_since(start_date.clone())
        .num_nanoseconds()
        .unwrap();
    let n = (n - 1) as i32;
    result.push(start_date.clone());
    for i in (0..n) {
        let d = (n as f64) / (i as f64 + 1.);
        let d = total_duration as f64 / d;
        let nd = start_date.clone() + Duration::nanoseconds(d as i64);
        result.push(nd);
    }
    // println!("{}", total_duration/n as i32);
    result
}

fn main() -> std::io::Result<()> {
    // Scatter Plots
    // simple_scatter_plot();
    chrono_plot();

    let start_date = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    // let start_date = start_date.naive_local();
    println!("{}", &start_date);

    let d1 = start_date + Duration::hours(6);
    // let d1 = Utc::now() + Duration::days(137);
    // let d1 = start_date.add(Duration::days(1));
    println!("{}", &d1);

    let end_date = Utc.ymd(2020, 1, 2).and_hms(0, 0, 0);
    let dates = date_range(start_date, end_date, 7);
    println!("{:?}", dates);

    Ok(())
}
