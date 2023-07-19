#![allow(dead_code)]

use ndarray::Array;
use plotly::{
    color::Rgb,
    common::{ColorScale, ColorScalePalette, Font, Marker, MarkerSymbol, Mode, Title},
    layout::{Axis, Camera, Layout, LayoutScene, Legend, Margin, ProjectionType},
    Mesh3D, Plot, Scatter3D, Surface,
};

// 3D Scatter Plots
fn simple_scatter3d_plot() {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec();
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
    let z: Vec<f64> = t.iter().map(|x| x.cos()).collect();

    let trace = Scatter3D::new(t, y, z).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();
}

fn customized_scatter3d_plot() {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec();
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
    let z: Vec<f64> = t.iter().map(|x| x.cos()).collect();
    let sizelookup = z.clone();

    let trace = Scatter3D::new(t.clone(), y.clone(), z.iter().map(|i| -i).collect())
        .name("Helix 1")
        .mode(Mode::Markers)
        .marker(
            Marker::new()
                .symbol(MarkerSymbol::Diamond)
                .size_array(
                    sizelookup
                        .iter()
                        .map(|i| (i.abs() * 25f64) as usize)
                        .collect(),
                )
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis)),
        );

    let trace2 = Scatter3D::new(t, z, y)
        .name("Helix 2")
        .mode(Mode::Markers)
        .marker(
            Marker::new()
                .size_array(
                    sizelookup
                        .iter()
                        .map(|i| (i.abs() * 25f64) as usize)
                        .collect(),
                )
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis)),
        );

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.add_trace(trace2);

    let background_color: Rgb = Rgb::new(70, 70, 70);
    let front_color: Rgb = Rgb::new(255, 255, 255);

    let layout = Layout::new()
        .title("Helix".into())
        .legend(Legend::new().x(0.9).y(0.9))
        .font(Font::new().color(front_color))
        .paper_background_color(background_color)
        .scene(
            LayoutScene::new()
                .x_axis(
                    Axis::new()
                        .title("x (A meaningful axis name goes here)".into())
                        .tick_angle(0f64)
                        .grid_color(front_color)
                        .color(front_color),
                )
                .y_axis(
                    Axis::new()
                        .title(Title::new("This is the label of the Y axis"))
                        .tick_format(".1f")
                        .grid_color(front_color)
                        .color(front_color),
                )
                .z_axis(Axis::new().title("".into()).tick_values(vec![]))
                .aspect_mode(plotly::layout::AspectMode::Manual)
                .aspect_ratio((3.0, 1.0, 1.0).into())
                .camera(
                    Camera::new()
                        .projection(ProjectionType::Orthographic.into())
                        .eye((0.0, 0.0, 1.0).into())
                        .up((0.0, 1.0, 0.0).into())
                        .center((0.0, 0.0, 0.0).into()),
                )
                .drag_mode(plotly::layout::DragMode3D::Pan)
                .hover_mode(plotly::layout::HoverMode::Closest)
                .background_color(background_color),
        )
        .margin(Margin::new().left(0).right(0).top(50).bottom(0))
        .width(1000)
        .height(500);
    plot.set_layout(layout);

    plot.show();
}

// 3D Line Plots
fn simple_line3d_plot() {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec();
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
    let z: Vec<f64> = t.iter().map(|x| x.cos()).collect();

    let trace = Scatter3D::new(t, y, z).mode(Mode::Lines);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();
}

// 3D Surface Plot
fn surface_plot() {
    let n: usize = 100;
    let x: Vec<f64> = Array::linspace(-10., 10., n).into_raw_vec();
    let y: Vec<f64> = Array::linspace(-10., 10., n).into_raw_vec();
    let z: Vec<Vec<f64>> = x
        .iter()
        .map(|i| {
            y.iter()
                .map(|j| 1.0 / (j * j + 5.0) * j.sin() + 1.0 / (i * i + 5.0) * i.cos())
                .collect()
        })
        .collect();

    let trace = Surface::new(z).x(x).y(y);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();
}

fn mesh_3d_plot() {
    let trace = Mesh3D::new(
        vec![0, 1, 2, 0],
        vec![0, 0, 1, 2],
        vec![0, 2, 0, 1],
        vec![0, 0, 0, 1],
        vec![1, 2, 3, 2],
        vec![2, 3, 1, 3],
    )
    .intensity(vec![0.0, 0.33, 0.66, 1.0])
    .color_scale(ColorScale::Palette(ColorScalePalette::Rainbow));

    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();
}

fn main() {
    // Uncomment any of these lines to display the example.

    // Scatter3D Plots
    // simple_scatter3d_plot();
    // simple_line3d_plot();
    // customized_scatter3d_plot();

    // Surface Plots
    // surface_plot();

    // Mesh Plots
    // mesh_3d_plot();
}
