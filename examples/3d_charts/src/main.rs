#![allow(dead_code)]

use ndarray::Array;
use plotly::{
    color::Rgb,
    common::{ColorBar, ColorScale, ColorScalePalette, Font, Marker, MarkerSymbol, Mode},
    layout::{Axis, Camera, Layout, LayoutScene, Legend, Margin, ProjectionType},
    Mesh3D, Plot, Scatter3D, Surface,
};
use rand::Rng;

// 3D Scatter Plots
fn simple_scatter3d_plot() {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec_and_offset().0;
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
    let z: Vec<f64> = t.iter().map(|x| x.cos()).collect();

    let trace = Scatter3D::new(t, y, z).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();
}

fn customized_scatter3d_plot() {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec_and_offset().0;
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
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                .color_array(z.clone()),
        );

    let trace2 = Scatter3D::new(t, z.clone(), y)
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
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                .color_array(z),
        );

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.add_trace(trace2);

    let background_color: Rgb = Rgb::new(70, 70, 70);
    let front_color: Rgb = Rgb::new(255, 255, 255);

    let layout = Layout::new()
        .title("Helix")
        .legend(Legend::new().x(0.9).y(0.9))
        .font(Font::new().color(front_color))
        .paper_background_color(background_color)
        .scene(
            LayoutScene::new()
                .x_axis(
                    Axis::new()
                        .title("x (A meaningful axis name goes here)")
                        .tick_angle(0f64)
                        .grid_color(front_color)
                        .color(front_color),
                )
                .y_axis(
                    Axis::new()
                        .title("This is the label of the Y axis")
                        .tick_format(".1f")
                        .grid_color(front_color)
                        .color(front_color),
                )
                .z_axis(Axis::new().title("").tick_values(vec![]))
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
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec_and_offset().0;
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
    let x: Vec<f64> = Array::linspace(-10., 10., n).into_raw_vec_and_offset().0;
    let y: Vec<f64> = Array::linspace(-10., 10., n).into_raw_vec_and_offset().0;
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

fn colorscale_plot() {
    let mut plot = Plot::new();

    let x = (0..100)
        .map(|x| ((x - 50) as f64) / 100f64)
        .collect::<Vec<f64>>();

    let y = x.clone();

    let iproduct = |x: &[f64], y: &[f64]| -> Vec<(f64, f64)> {
        let mut result = Vec::new();
        for x in x {
            for y in y {
                result.push((*x, *y));
            }
        }
        result
    };

    let ((x, y), z): ((Vec<f64>, Vec<f64>), Vec<f64>) = iproduct(&x, &y)
        .into_iter()
        .map(|(x, y)| ((x, y), -(x.powi(2) + y.powi(2)) + 0.5))
        .unzip();

    let color: Vec<f32> = z.clone().into_iter().rev().map(|x| x as f32).collect();
    let _color: Vec<usize> = (0..z.len()).collect();
    let _color: Vec<u8> = (0..z.len()).map(|x| x as u8).collect();
    let _color: Vec<i16> = {
        let mut rng = rand::thread_rng();
        (0..z.len()).map(|_| rng.gen_range(0..100)).collect()
    };

    let color_max = color.iter().fold(f64::MIN, |acc, x| acc.max(*x as f64));

    let colorscale = ColorScalePalette::YlGnBu;

    let marker = Marker::new()
        .color_array(color)
        .color_scale(plotly::common::ColorScale::Palette(colorscale.clone()))
        .cauto(false)
        .cmax(color_max * 1.5)
        .color_bar(ColorBar::new());

    let scatter = Scatter3D::new(x, y, z).mode(Mode::Markers).marker(marker);

    plot.add_trace(scatter);

    let layout = Layout::new()
        .font(Font::new().size(18).family("Palatino-Linotype"))
        .title(format!("Colorscale: {colorscale:?}"))
        .width(1200)
        .height(1000)
        .scene(
            LayoutScene::new()
                .aspect_mode(plotly::layout::AspectMode::Data)
                .x_axis(Axis::new().tick_format(".1f"))
                .y_axis(Axis::new().tick_format(".1f"))
                .z_axis(Axis::new().tick_format(".1f")),
        );

    plot.set_layout(layout);

    plot.show();
}

fn main() {
    // Uncomment any of these lines to display the example.

    // Scatter3D Plots
    // simple_scatter3d_plot();
    // simple_line3d_plot();
    // customized_scatter3d_plot();
    // colorscale_plot();

    // Surface Plots
    // surface_plot();

    // Mesh Plots
    // mesh_3d_plot();
}
