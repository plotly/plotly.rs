mod layout;
mod page;
mod plotly_chart;

use ndarray::Array;
use plotly::{
    color::NamedColor,
    common::{Marker, Mode, Title},
    Configuration, Layout, Plot, Scatter, Scatter3D,
};
use proa_core::HeadConfig;
use proa_framework_core::render_sync_html_document;

use crate::{
    layout::PlotLayout,
    page::{Chart, MultiplePlotsPage},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let charts = vec![
        Chart::new("scatter_1", "Sine wave", &first_plot())?,
        Chart::new("scatter_2", "Colored markers", &second_plot())?,
        Chart::new("scatter_3_3d", "3D scatter", &third_plot())?,
    ];
    let html = render_sync_html_document(
        PlotLayout {
            class: None,
            children: Some(MultiplePlotsPage::new(charts)),
        },
        HeadConfig::default(),
    )?;

    std::fs::create_dir_all("./output")?;
    std::fs::write("./output/multiple_plots.html", html)?;
    println!("Multiple plots HTML page saved to: ./output/multiple_plots.html");
    Ok(())
}

// These construct the same figures as customization/multiple_plots_example.
// They return Plot values, leaving HTML and library loading to the components.
fn first_plot() -> Plot {
    let t: Vec<f64> = Array::linspace(0., 10., 100).into_raw_vec_and_offset().0;
    let y = t.iter().map(|x| x.sin()).collect();
    let mut plot = Plot::new();
    plot.add_trace(Scatter::new(t, y).mode(Mode::Markers));
    plot.set_configuration(Configuration::default().responsive(true));
    plot
}

fn second_plot() -> Plot {
    let trace = Scatter::new(vec![1, 2, 3, 4], vec![10, 11, 12, 13])
        .mode(Mode::Markers)
        .marker(
            Marker::new()
                .size_array(vec![40, 60, 80, 100])
                .color_array(vec![
                    NamedColor::Red,
                    NamedColor::Blue,
                    NamedColor::Cyan,
                    NamedColor::OrangeRed,
                ]),
        );
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_configuration(Configuration::default().responsive(true));
    plot
}

fn third_plot() -> Plot {
    let t: Vec<f64> = Array::linspace(0., 10., 100).into_raw_vec_and_offset().0;
    let y = t.iter().map(|x| x.sin()).collect();
    let z = t.iter().map(|x| x.cos()).collect();
    let mut plot = Plot::new();
    plot.add_trace(Scatter3D::new(t, y, z).mode(Mode::Markers));
    plot.set_layout(Layout::new().title(Title::with_text("Scatter3d")));
    plot.set_configuration(Configuration::default().responsive(true));
    plot
}
