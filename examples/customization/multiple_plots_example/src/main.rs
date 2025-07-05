use std::fs::File;
use std::io::Write;

use build_html::*;
use ndarray::Array;
use plotly::{
    color::NamedColor,
    common::{Marker, Mode, Title},
    Layout, Plot, Scatter, Scatter3D,
};

fn main() {
    let html: String = HtmlPage::new()
        .with_title("Plotly-rs Multiple Plots")
        .with_script_link("https://cdn.plot.ly/plotly-latest.min.js")
        .with_header(1, "Multiple Plotly plots on the same HTML page")
        .with_raw(first_plot())
        .with_raw(second_plot())
        .with_raw(third_plot())
        .to_html_string();

    std::fs::create_dir_all("./output").unwrap();
    let path = "./output/multiple_plots.html";
    let mut file = File::create(path).unwrap();
    file.write_all(html.as_bytes())
        .expect("failed to write html output");
    file.flush().unwrap();

    println!("Multiple plots HTML page saved to: {path}");
}

fn first_plot() -> String {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec_and_offset().0;
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();

    let trace = Scatter::new(t, y).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.to_inline_html(Some("scatter_1"))
}

fn second_plot() -> String {
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
    plot.to_inline_html(Some("scatter_2"))
}

fn third_plot() -> String {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec_and_offset().0;
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
    let z: Vec<f64> = t.iter().map(|x| x.cos()).collect();

    let trace = Scatter3D::new(t, y, z).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    let l = Layout::new()
        .title(Title::with_text("Scatter3d"))
        .height(800);
    plot.set_layout(l);
    plot.to_inline_html(Some("scatter_3_3d"))
}
