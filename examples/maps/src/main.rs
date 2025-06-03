#![allow(dead_code)]

use plotly::{
    common::Marker,
    layout::{Center, DragMode, Mapbox, MapboxStyle, Margin},
    DensityMapbox, Layout, Plot, ScatterMapbox,
};

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

fn write_example_to_html(plot: &Plot, name: &str) -> String {
    std::fs::create_dir_all("./output").unwrap();
    let path = format!("./output/{}.html", name);
    plot.write_html(&path);
    path
}

fn main() {
    // Change false to true on any of these lines to display the example.
    scatter_mapbox(false, "scatter_mapbox");
    density_mapbox(false, "density_mapbox");
}
