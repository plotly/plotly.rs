#![allow(dead_code)]

use ndarray::Array;
use plotly::{
    common::{ColorScale, ColorScalePalette, Marker, MarkerSymbol, Mode, Title},
    layout::{Axis, Layout},
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

    let trace2 = Scatter3D::new(t, z, y).mode(Mode::Markers).marker(
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
    let layout = Layout::new()
        .title("Helix".into())
        .x_axis(Axis::new().title("x (A meaningful axis name goes here)".into()))
        .y_axis(Axis::new().title(Title::new("This is the label of the Y axis")))
        .z_axis(Axis::new().title("z Axis".into()));
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
