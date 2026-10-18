#![allow(dead_code)]

use ndarray::Array;
use plotly::{
    color::Rgb,
    common::{ColorBar, ColorScale, ColorScalePalette, Font, Marker, MarkerSymbol, Mode},
    isosurface::{Cap, Caps, IsoSurfaceSurface},
    layout::{Axis, Camera, Layout, LayoutScene, Legend, Margin, ProjectionType},
    stream_tube::Starts,
    volume::OpacityScale,
    Cone, IsoSurface, Mesh3D, Plot, Scatter3D, StreamTube, Surface, Volume,
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

// ANCHOR: grid_3d
/// Flattened coordinates of an `n`x`n`x`n` grid spanning `[lo, hi]` on each
/// axis, in the x-fastest order expected by volumetric traces.
fn grid_3d(n: usize, lo: f64, hi: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let step = (hi - lo) / (n - 1) as f64;
    let (mut x, mut y, mut z) = (Vec::new(), Vec::new(), Vec::new());
    for k in 0..n {
        for j in 0..n {
            for i in 0..n {
                x.push(lo + i as f64 * step);
                y.push(lo + j as f64 * step);
                z.push(lo + k as f64 * step);
            }
        }
    }
    (x, y, z)
}
// ANCHOR_END: grid_3d

// Cone Plots
// ANCHOR: cone_plot
fn cone_plot(show: bool, file_name: &str) {
    // A vortex: vectors rotate around the z axis and drift upwards.
    let (x, y, z) = grid_3d(5, -1.0, 1.0);
    let u: Vec<f64> = y.iter().map(|y| -y).collect();
    let v: Vec<f64> = x.clone();
    let w: Vec<f64> = vec![0.3; x.len()];

    let trace = Cone::new(x, y, z, u, v, w)
        .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
        .size_ref(0.5)
        .color_bar(ColorBar::new().title("Norm"));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(
        Layout::new()
            .title("Cone plot of a vortex")
            .scene(LayoutScene::new().camera(Camera::new())),
    );

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: cone_plot

// Streamtube Plots
// ANCHOR: streamtube_plot
fn streamtube_plot(show: bool, file_name: &str) {
    // Flow along x with a sinusoidal swirl in y and z.
    let (x, y, z) = grid_3d(8, 0.0, 4.0);
    let u: Vec<f64> = vec![1.0; x.len()];
    let v: Vec<f64> = x.iter().map(|x| 0.5 * x.sin()).collect();
    let w: Vec<f64> = x.iter().map(|x| 0.5 * x.cos()).collect();

    let starts = Starts::new()
        .x(vec![0.0; 4])
        .y(vec![1.0, 1.0, 3.0, 3.0])
        .z(vec![1.0, 3.0, 1.0, 3.0]);

    let trace = StreamTube::new(x, y, z, u, v, w)
        .starts(starts)
        .size_ref(0.5)
        .color_scale(ColorScale::Palette(ColorScalePalette::Portland))
        .show_scale(false);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(Layout::new().title("Streamtubes following a swirling flow"));

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: streamtube_plot

// Isosurface Plots
// ANCHOR: isosurface_plot
fn isosurface_plot(show: bool, file_name: &str) {
    let (x, y, z) = grid_3d(20, -1.0, 1.0);
    let value: Vec<f64> = x
        .iter()
        .zip(&y)
        .zip(&z)
        .map(|((x, y), z)| x * x + y * y + z * z)
        .collect();

    let trace = IsoSurface::new(x, y, z, value)
        .iso_min(0.2)
        .iso_max(0.8)
        .surface(IsoSurfaceSurface::new().count(3))
        .caps(
            Caps::new()
                .x(Cap::new().show(false))
                .y(Cap::new().show(false))
                .z(Cap::new().show(false)),
        )
        .opacity(0.6)
        .color_scale(ColorScale::Palette(ColorScalePalette::Blues));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(Layout::new().title("Nested spherical isosurfaces"));

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: isosurface_plot

// Volume Plots
// ANCHOR: volume_plot
fn volume_plot(show: bool, file_name: &str) {
    let (x, y, z) = grid_3d(20, -1.0, 1.0);
    let value: Vec<f64> = x
        .iter()
        .zip(&y)
        .zip(&z)
        .map(|((x, y), z)| (3.0 * x).sin() * (3.0 * y).cos() * (3.0 * z).sin())
        .collect();

    let trace = Volume::new(x, y, z, value)
        .iso_min(-0.5)
        .iso_max(0.5)
        .opacity(0.1)
        .opacity_scale(OpacityScale::Extremes)
        .surface(IsoSurfaceSurface::new().count(15))
        .color_scale(ColorScale::Palette(ColorScalePalette::RdBu));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(Layout::new().title("Volume rendering"));

    let path = write_example_to_html(&plot, file_name);
    if show {
        plot.show_html(path);
    }
}
// ANCHOR_END: volume_plot

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

    // Vector Field Plots
    cone_plot(false, "cone_plot");
    streamtube_plot(false, "streamtube_plot");

    // Volume Plots
    isosurface_plot(false, "isosurface_plot");
    volume_plot(false, "volume_plot");
}
