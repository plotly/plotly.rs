#![allow(dead_code)]

use plotly::{
    common::Marker,
    layout::{Center, DragMode, Mapbox, MapboxStyle, Margin},
    Layout, Plot, ScatterMapbox,
};

fn scatter_mapbox() {
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

    plot.show();
}

fn main() {
    // Uncomment any of these lines to display the example.

    // scatter_mapbox();
}
