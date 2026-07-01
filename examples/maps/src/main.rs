#![allow(dead_code)]

use plotly::{
    choropleth::{LocationMode, Marker as ChoroplethMarker},
    color::Rgb,
    common::{ColorBar, ColorScale, ColorScalePalette, Line, Marker, Mode},
    layout::{
        Axis, Center, DragMode, GeoResolution, LayoutGeo, LayoutMap, MapStyle, Mapbox, MapboxStyle,
        Projection, Rotation,
    },
    Choropleth, ChoroplethMap, Configuration, DensityMapbox, Layout, Plot, ScatterGeo,
    ScatterMapbox,
};
use plotly_utils::write_example_to_html;

fn scatter_mapbox(show: bool, file_name: &str) {
    let trace = ScatterMapbox::new(vec![45.5017], vec![-73.5673])
        .marker(Marker::new().size(25).opacity(0.9));

    let layout = Layout::new().drag_mode(DragMode::Zoom).mapbox(
        Mapbox::new()
            .style(MapboxStyle::OpenStreetMap)
            .center(Center::new(45.5017, -73.5673))
            .zoom(5),
    );

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.set_configuration(Configuration::default().responsive(true).fill_frame(true));

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

/// Reproduce the Earth from https://plotly.com/javascript/lines-on-maps/#lines-on-an-orthographic-map
fn scatter_geo(show: bool, file_name: &str) {
    use csv;
    use reqwest;

    // Download and parse the CSV. If the fetch fails (e.g. no network during a
    // book/CI build), warn and skip this example rather than panicking.
    let url = "https://raw.githubusercontent.com/plotly/datasets/master/globe_contours.csv";
    let req = match reqwest::blocking::get(url)
        .and_then(|resp| resp.error_for_status())
        .and_then(|resp| resp.text())
    {
        Ok(body) => body,
        Err(err) => {
            eprintln!("warning: skipping scatter_geo example; failed to fetch {url}: {err}");
            return;
        }
    };
    let mut rdr = csv::Reader::from_reader(req.as_bytes());
    let headers = match rdr.headers() {
        Ok(headers) => headers.clone(),
        Err(err) => {
            eprintln!("warning: skipping scatter_geo example; failed to read CSV headers: {err}");
            return;
        }
    };
    let mut rows = vec![];
    for result in rdr.records() {
        match result {
            Ok(record) => rows.push(record),
            Err(err) => {
                eprintln!(
                    "warning: skipping scatter_geo example; failed to parse CSV record: {err}"
                );
                return;
            }
        }
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
        let (lat_idx, lon_idx) = match (
            headers.iter().position(|h| h == lat_head),
            headers.iter().position(|h| h == lon_head),
        ) {
            (Some(lat_idx), Some(lon_idx)) => (lat_idx, lon_idx),
            _ => {
                eprintln!(
                    "warning: skipping scatter_geo example; missing expected columns \
                     {lat_head}/{lon_head}"
                );
                return;
            }
        };
        let lat: Vec<f64> = rows
            .iter()
            .map(|row| row.get(lat_idx).unwrap_or("").parse().unwrap_or(f64::NAN))
            .collect();
        let lon: Vec<f64> = rows
            .iter()
            .map(|row| row.get(lon_idx).unwrap_or("").parse().unwrap_or(f64::NAN))
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

    let layout = Layout::new().drag_mode(DragMode::Zoom).geo(
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

    plot.set_configuration(Configuration::default().responsive(true).fill_frame(true));
    plot.set_layout(layout);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

fn density_mapbox(show: bool, file_name: &str) {
    let trace = DensityMapbox::new(vec![45.5017], vec![-73.5673], vec![0.75]).zauto(true);

    let layout = Layout::new().drag_mode(DragMode::Zoom).mapbox(
        Mapbox::new()
            .style(MapboxStyle::OpenStreetMap)
            .center(Center::new(45.5017, -73.5673))
            .zoom(5),
    );

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.set_configuration(Configuration::default().responsive(true).fill_frame(true));

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

/// Classic choropleth on the `geo` subplot, coloring countries by value using
/// ISO-3 country codes.
// ANCHOR: choropleth
fn choropleth(show: bool, file_name: &str) {
    let trace = Choropleth::new(
        vec![
            "USA", "CAN", "MEX", "BRA", "ARG", "FRA", "DEU", "CHN", "IND", "AUS",
        ],
        vec![10.0, 8.0, 6.0, 7.0, 4.0, 9.0, 9.5, 12.0, 11.0, 5.0],
    )
    .location_mode(LocationMode::Iso3)
    .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
    .color_bar(ColorBar::new().title("Score"))
    .marker(ChoroplethMarker::new().line(Line::new().width(0.5).color(Rgb::new(80, 80, 80))));

    let layout = Layout::new().drag_mode(DragMode::Zoom).geo(
        LayoutGeo::new()
            .showcountries(true)
            .showland(true)
            .resolution(GeoResolution::OneOverFiftyMillion),
    );

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.set_configuration(Configuration::default().responsive(true).fill_frame(true));

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: choropleth

/// Choropleth on the MapLibre `map` subplot. Regions are matched against a
/// GeoJSON feature collection (referenced here by URL) via `feature_id_key`.
// ANCHOR: choropleth_map
fn choropleth_map(show: bool, file_name: &str) {
    let geojson_url =
        "https://raw.githubusercontent.com/python-visualization/folium/main/tests/us-states.json";

    let trace = ChoroplethMap::new(
        vec!["AL", "AK", "AZ", "CA", "NY", "TX"],
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    )
    .geojson(serde_json::json!(geojson_url))
    .feature_id_key("id")
    .color_scale(ColorScale::Palette(ColorScalePalette::Bluered))
    .show_scale(true)
    .marker(ChoroplethMarker::new().opacity(0.7));

    let layout = Layout::new().drag_mode(DragMode::Zoom).map(
        LayoutMap::new()
            .style(MapStyle::CartoPositron)
            .center(Center::new(38.0, -96.0))
            .zoom(3.0),
    );

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.set_configuration(Configuration::default().responsive(true).fill_frame(true));

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: choropleth_map

fn main() {
    // Change false to true on any of these lines to display the example.
    scatter_mapbox(false, "scatter_mapbox");
    scatter_geo(false, "scatter_geo");
    density_mapbox(false, "density_mapbox");
    choropleth(false, "choropleth");
    choropleth_map(false, "choropleth_map");
}
