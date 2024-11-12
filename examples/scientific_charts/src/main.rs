#![allow(dead_code)]

use std::f64::consts::PI;

use plotly::common::{ColorScale, ColorScalePalette, Font};
use plotly::contour::Contours;
use plotly::{Contour, HeatMap, Layout, Plot};

// Contour Plots
fn simple_contour_plot() {
    let n = 200;
    let mut x = Vec::<f64>::new();
    let mut y = Vec::<f64>::new();
    let mut z: Vec<Vec<f64>> = Vec::new();

    for index in 0..n {
        let value = -2.0 * PI + 4.0 * PI * (index as f64) / (n as f64);
        x.push(value);
        y.push(value);
    }

    y.iter().take(n).for_each(|y| {
        let mut row = Vec::<f64>::new();
        x.iter().take(n).for_each(|x| {
            let radius_squared = x.powf(2.0) + y.powf(2.0);
            let zv = x.sin() * y.cos() * radius_squared.sin() / (radius_squared + 1.0).log10();
            row.push(zv);
        });
        z.push(row);
    });

    let trace = Contour::new(x, y, z);
    let mut plot = Plot::new();

    plot.add_trace(trace);

    plot.show();
}

fn colorscale_for_contour_plot() {
    let z = vec![
        vec![10.0, 10.625, 12.5, 15.625, 20.0],
        vec![5.625, 6.25, 8.125, 11.25, 15.625],
        vec![2.5, 3.125, 5., 8.125, 12.5],
        vec![0.625, 1.25, 3.125, 6.25, 10.625],
        vec![0.0, 0.625, 2.5, 5.625, 10.0],
    ];
    let trace = Contour::new_z(z).color_scale(ColorScale::Palette(ColorScalePalette::Jet));

    let layout = Layout::new().title("Colorscale for Contour Plot");
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace);

    plot.show();
}

fn customizing_size_and_range_of_a_contour_plots_contours() {
    let z = vec![
        vec![10.0, 10.625, 12.5, 15.625, 20.0],
        vec![5.625, 6.25, 8.125, 11.25, 15.625],
        vec![2.5, 3.125, 5., 8.125, 12.5],
        vec![0.625, 1.25, 3.125, 6.25, 10.625],
        vec![0.0, 0.625, 2.5, 5.625, 10.0],
    ];
    let trace = Contour::new_z(z)
        .color_scale(ColorScale::Palette(ColorScalePalette::Jet))
        .auto_contour(false)
        .contours(Contours::new().start(0.0).end(8.0).size(2.0));

    let layout = Layout::new().title("Customizing Size and Range of Contours");
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace);

    plot.show();
}

fn customizing_spacing_between_x_and_y_ticks() {
    let z = vec![
        vec![10.0, 10.625, 12.5, 15.625, 20.0],
        vec![5.625, 6.25, 8.125, 11.25, 15.625],
        vec![2.5, 3.125, 5., 8.125, 12.5],
        vec![0.625, 1.25, 3.125, 6.25, 10.625],
        vec![0.0, 0.625, 2.5, 5.625, 10.0],
    ];
    let trace = Contour::new_z(z)
        .color_scale(ColorScale::Palette(ColorScalePalette::Jet))
        .dx(10.0)
        .x0(5.0)
        .dy(10.0)
        .y0(10.0);

    let layout = Layout::new().title("Customizing Size and Range of Contours");
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace);

    plot.show();
}

// Heatmaps
fn basic_heat_map() {
    let z = vec![vec![1, 20, 30], vec![20, 1, 60], vec![30, 60, 1]];
    let trace = HeatMap::new_z(z).zmin(1.0).zmax(60.0);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show();
}

fn customized_heat_map() {
    let x = (0..100).map(|x| x as f64).collect::<Vec<f64>>();
    let y = (0..100).map(|y| y as f64).collect::<Vec<f64>>();
    let z: Vec<Vec<f64>> = y
        .iter()
        .map(|y| {
            x.iter()
                .map(|x| (x / 5.0).powf(2.0) + (y / 5.0).powf(2.0))
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();

    let (_z_min, z_max) = z
        .iter()
        .flatten()
        .fold((f64::MAX, f64::MIN), |(min, max), &x| {
            (min.min(x), max.max(x))
        });

    let colorscale = ColorScalePalette::Jet;

    let trace = HeatMap::new(x, y, z)
        .zmin(z_max * 0.4)
        .zmax(z_max * 0.5)
        .color_scale(colorscale.into());

    let layout = Layout::new()
        .title("Customized Heatmap")
        .font(Font::new().size(32));

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace);

    plot.show();
}

fn main() {
    // Uncomment any of these lines to display the example.

    // Contour Plots
    // simple_contour_plot();
    // colorscale_for_contour_plot();
    // customizing_size_and_range_of_a_contour_plots_contours();
    // customizing_spacing_between_x_and_y_ticks();

    // Heatmaps
    // basic_heat_map();
    // customized_heat_map();
}
