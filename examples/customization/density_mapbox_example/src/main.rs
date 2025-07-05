use plotly::{
    layout::{Center, DragMode, Mapbox, MapboxStyle, Margin},
    Configuration, DensityMapbox, Layout, Plot,
};
use plotly_utils::write_example_to_html;

fn main() {
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
    plot.set_configuration(Configuration::default().responsive(true).fill_frame(true));

    let path = write_example_to_html(&plot, "density_mapbox");
    println!("Density mapbox plot saved to: {path}");
}
