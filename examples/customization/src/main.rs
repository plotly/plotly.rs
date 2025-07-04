#![allow(dead_code)]

use std::fs::File;
use std::io::Write;

use build_html::*;
use ndarray::Array;
use plotly::{
    color::NamedColor,
    common::{Marker, Mode, Title},
    layout::{Center, DragMode, Mapbox, MapboxStyle, Margin},
    Configuration, DensityMapbox, Layout, Plot, Scatter, Scatter3D,
};
use plotly_utils::write_example_to_html;
const DEFAULT_HTML_APP_NOT_FOUND: &str = "Could not find default application for HTML files.";

fn density_mapbox_responsive_autofill(show: bool, file_name: &str) {
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}

fn multiple_plots_on_same_html_page(show: bool, file_name: &str) {
    let html: String = HtmlPage::new()
        .with_title("Plotly-rs Multiple Plots")
        .with_script_link("https://cdn.plot.ly/plotly-latest.min.js")
        .with_header(1, "Multiple Plotly plots on the same HTML page")
        .with_raw(first_plot())
        .with_raw(second_plot())
        .with_raw(third_plot())
        .to_html_string();

    std::fs::create_dir_all("./output").unwrap();
    let path = format!("./output/inline_{file_name}.html");
    let mut file = File::create(&path).unwrap();
    file.write_all(html.as_bytes())
        .expect("failed to write html output");
    file.flush().unwrap();
    if show {
        show_with_default_app(&path);
    }
}

fn first_plot() -> String {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec_and_offset().0;
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();

    let trace = Scatter::new(t, y).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.to_inline_html(Some("scattter_1"))
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

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
fn show_with_default_app(temp_path: &str) {
    use std::process::Command;
    Command::new("xdg-open")
        .args([temp_path])
        .output()
        .expect(DEFAULT_HTML_APP_NOT_FOUND);
}

#[cfg(target_os = "macos")]
fn show_with_default_app(temp_path: &str) {
    use std::process::Command;
    Command::new("open")
        .args([temp_path])
        .output()
        .expect(DEFAULT_HTML_APP_NOT_FOUND);
}

#[cfg(target_os = "windows")]
fn show_with_default_app(temp_path: &str) {
    use std::process::Command;
    Command::new("cmd")
        .args(&["/C", "start", &format!(r#"{}"#, temp_path)])
        .spawn()
        .expect(DEFAULT_HTML_APP_NOT_FOUND);
}

fn main() {
    // Switch the boolean flag to `true` to display the example, otherwise manually
    // open the generated file in the `output` folder.
    density_mapbox_responsive_autofill(false, "density_mapbox");
    multiple_plots_on_same_html_page(false, "multiple_plots");
}
