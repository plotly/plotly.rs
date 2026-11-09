use ndarray::Array;
use plotly::{
    color::NamedColor,
    common::{Marker, Mode, Title},
    Configuration, Layout, Plot, Scatter, Scatter3D,
};

// These construct the same figures as customization/multiple_plots_example.
// They return Plot values, leaving HTML and library loading to the components.
pub fn first_plot() -> Plot {
    let t: Vec<f64> = Array::linspace(0., 10., 100).into_raw_vec_and_offset().0;
    let y = t.iter().map(|x| x.sin()).collect();
    let mut plot = Plot::new();
    plot.add_trace(Scatter::new(t, y).mode(Mode::Markers));
    plot.set_configuration(Configuration::default().responsive(true));
    plot
}

pub fn second_plot() -> Plot {
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

pub fn third_plot() -> Plot {
    let t: Vec<f64> = Array::linspace(0., 10., 100).into_raw_vec_and_offset().0;
    let y = t.iter().map(|x| x.sin()).collect();
    let z = t.iter().map(|x| x.cos()).collect();
    let mut plot = Plot::new();
    plot.add_trace(Scatter3D::new(t, y, z).mode(Mode::Markers));
    plot.set_layout(Layout::new().title(Title::with_text("Scatter3d")));
    plot.set_configuration(Configuration::default().responsive(true));
    plot
}
