#![allow(dead_code)]

use plotly::{
    color::Rgb,
    common::{Line, Marker, Mode},
    layout::{
        Axis, Center, DragMode, LayoutGeo, Mapbox, MapboxStyle, Margin, Projection, Rotation,
    },
    DensityMapbox, Layout, Plot, ScatterGeo, ScatterMapbox,
};
use plotly_utils::write_example_to_html;

fn scatter_mapbox(show: bool, file_name: &str) {
    let trace = ScatterMapbox::new(vec![45.5017], vec![-73.5673])
        .marker(Marker::new().size(25).opacity(0.9));

    let layout = Layout::new()
        .drag_mode(DragMode::Zoom)
        .margin(Margin::new().top(0).left(0).bottom(0).right(0))
        .mapbox(
            Mapbox::new()
                .style(MapboxStyle::OpenStreetMap)
                .center(Center::new(45.5017, -73.5673))
                .zoom(5),
        );

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

/// Reproduce the Earth from https://plotly.com/javascript/lines-on-maps/#lines-on-an-orthographic-map
fn scatter_geo(show: bool, file_name: &str) {
    use csv;
    use reqwest;

    // Download and parse the CSV
    let url = "https://raw.githubusercontent.com/plotly/datasets/master/globe_contours.csv";
    let req = reqwest::blocking::get(url).unwrap().text().unwrap();
    let mut rdr = csv::Reader::from_reader(req.as_bytes());
    let headers = rdr.headers().unwrap().clone();
    let mut rows = vec![];
    for result in rdr.records() {
        let record = result.unwrap();
        rows.push(record);
    }

    // Color scale
    let scl = [
        "rgb(213,62,79)",
        "rgb(244,109,67)",
        "rgb(253,174,97)",
        "rgb(254,224,139)",
        "rgb(255,255,191)",
        "rgb(230,245,152)",
        "rgb(171,221,164)",
        "rgb(102,194,165)",
        "rgb(50,136,189)",
    ];

    // Unpack lat/lon columns
    let mut all_lats: Vec<Vec<f64>> = vec![];
    let mut all_lons: Vec<Vec<f64>> = vec![];
    for i in 0..scl.len() {
        let lat_head = format!("lat-{}", i + 1);
        let lon_head = format!("lon-{}", i + 1);
        let lat: Vec<f64> = rows
            .iter()
            .map(|row| {
                row.get(headers.iter().position(|h| h == lat_head).unwrap())
                    .unwrap()
                    .parse()
                    .unwrap_or(f64::NAN)
            })
            .collect();
        let lon: Vec<f64> = rows
            .iter()
            .map(|row| {
                row.get(headers.iter().position(|h| h == lon_head).unwrap())
                    .unwrap()
                    .parse()
                    .unwrap_or(f64::NAN)
            })
            .collect();
        all_lats.push(lat);
        all_lons.push(lon);
    }

    // Build traces
    let mut plot = Plot::new();
    for i in 0..scl.len() {
        let trace = ScatterGeo::new(all_lats[i].clone(), all_lons[i].clone())
            .mode(Mode::Lines)
            .line(Line::new().width(2.0).color(scl[i]));
        plot.add_trace(trace);
    }

    let layout = Layout::new()
        .drag_mode(DragMode::Zoom)
        .margin(Margin::new().top(0).left(0).bottom(0).right(0))
        .geo(
            LayoutGeo::new()
                .showocean(true)
                .showlakes(true)
                .showcountries(true)
                .showland(true)
                .oceancolor(Rgb::new(0, 255, 255))
                .lakecolor(Rgb::new(0, 255, 255))
                .landcolor(Rgb::new(230, 145, 56))
                .lataxis(
                    Axis::new()
                        .show_grid(true)
                        .grid_color(Rgb::new(102, 102, 102)),
                )
                .lonaxis(
                    Axis::new()
                        .show_grid(true)
                        .grid_color(Rgb::new(102, 102, 102)),
                )
                .projection(
                    Projection::new()
                        .projection_type(plotly::layout::ProjectionType::Orthographic)
                        .rotation(Rotation::new().lon(-100.0).lat(40.0)),
                ),
        );

    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

fn density_mapbox(show: bool, file_name: &str) {
    let trace = DensityMapbox::new(vec![45.5017], vec![-73.5673], vec![0.75]).zauto(true);

    let layout = Layout::new()
        .drag_mode(DragMode::Zoom)
        .margin(Margin::new().top(0).left(0).bottom(0).right(0))
        .mapbox(
            Mapbox::new()
                .style(MapboxStyle::OpenStreetMap)
                .center(Center::new(45.5017, -73.5673))
                .zoom(5),
        );

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

fn main() {
    // Change false to true on any of these lines to display the example.
    scatter_mapbox(false, "scatter_mapbox");
    scatter_geo(true, "scatter_geo");
    density_mapbox(false, "density_mapbox");
}
