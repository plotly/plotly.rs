#![allow(dead_code)]

use ndarray::Array;
use plotly::{
    color::Rgb,
    common::{ColorBar, ColorScale, ColorScalePalette, Font, Marker, MarkerSymbol, Mode},
    layout::{Axis, Camera, Layout, LayoutScene, Legend, Margin, ProjectionType},
    Mesh3D, Plot, Scatter3D, Surface,
};
use plotly_utils::write_example_to_html;
use rand::RngExt;

// 3D Scatter Plots
// ANCHOR: simple_scatter3d_plot
fn simple_scatter3d_plot(show: bool, file_name: &str) {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec_and_offset().0;
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
    let z: Vec<f64> = t.iter().map(|x| x.cos()).collect();

    let trace = Scatter3D::new(t, y, z).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: simple_scatter3d_plot

// ANCHOR: customized_scatter3d_plot
fn customized_scatter3d_plot(show: bool, file_name: &str) {
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: customized_scatter3d_plot

// 3D Line Plots
// ANCHOR: simple_line3d_plot
fn simple_line3d_plot(show: bool, file_name: &str) {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec_and_offset().0;
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
    let z: Vec<f64> = t.iter().map(|x| x.cos()).collect();

    let trace = Scatter3D::new(t, y, z).mode(Mode::Lines);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: simple_line3d_plot

// 3D Surface Plot
// ANCHOR: surface_plot
fn surface_plot(show: bool, file_name: &str) {
    let n: usize = 100;
    let x: Vec<f64> = Array::linspace(-10., 10., n).into_raw_vec_and_offset().0;
    let y: Vec<f64> = Array::linspace(-10., 10., n).into_raw_vec_and_offset().0;
    let z: Vec<Vec<f64>> = y
        .iter()
        .map(|i| {
            x.iter()
                .map(|j| 1.0 / (j * j + 5.0) * j.sin() + 1.0 / (i * i + 5.0) * i.cos())
                .collect()
        })
        .collect();

    let trace = Surface::new(z).x(x).y(y);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: surface_plot

// ANCHOR: mesh_3d_plot
fn mesh_3d_plot(show: bool, file_name: &str) {
    let trace = Mesh3D::new(
        vec![0, 1, 2, 0],
        vec![0, 0, 1, 2],
        vec![0, 2, 0, 1],
        Some(vec![0, 0, 0, 1]),
        Some(vec![1, 2, 3, 2]),
        Some(vec![2, 3, 1, 3]),
    )
    .intensity(vec![0.0, 0.33, 0.66, 1.0])
    .color_scale(ColorScale::Palette(ColorScalePalette::Rainbow));

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: mesh_3d_plot

// ANCHOR: colorscale_plot
fn colorscale_plot(show: bool, file_name: &str) {
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
        let mut rng = rand::rng();
        (0..z.len()).map(|_| rng.random_range(0..100)).collect()
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

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: colorscale_plot

fn main() {
    // Change false to true on any of these lines to display the example.
    // Scatter3D Plots
    simple_scatter3d_plot(false, "simple_scatter3d_plot");
    simple_line3d_plot(false, "simple_line3d_plot");
    customized_scatter3d_plot(false, "customized_scatter3d_plot");
    colorscale_plot(false, "colorscale_plot");

    // Surface Plots
    surface_plot(false, "surface_plot");

    // Mesh Plots
    mesh_3d_plot(false, "mesh_3d_plot");
}
