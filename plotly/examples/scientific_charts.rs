use std::f64::consts::PI;

use plotly::common::{ColorScale, ColorScalePalette, Title};
use plotly::contour::Contours;
use plotly::{Contour, HeatMap, Layout, Plot};

// Contour Plots
fn simple_contour_plot(show: bool) {
    let n = 200;
    let mut x = Vec::<f64>::new();
    let mut y = Vec::<f64>::new();
    let mut z: Vec<Vec<f64>> = Vec::new();

    for index in 0..n {
        let value = -2.0 * PI + 4.0 * PI * (index as f64) / (n as f64);
        x.push(value);
        y.push(value);
    }

    x.iter().take(n).for_each(|x| {
        let mut row = Vec::<f64>::new();
        y.iter().take(n).for_each(|y| {
            let radius_squared = x.powf(2.0) + y.powf(2.0);
            let zv = x.sin() * y.cos() * radius_squared.sin() / (radius_squared + 1.0).log10();
            row.push(zv);
        });
        z.push(row);
    });

    let trace = Contour::new(x, y, z);
    let mut plot = Plot::new();

    plot.add_trace(trace);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("simple_contour_plot")));
}

fn colorscale_for_contour_plot(show: bool) {
    let z = vec![
        vec![10.0, 10.625, 12.5, 15.625, 20.0],
        vec![5.625, 6.25, 8.125, 11.25, 15.625],
        vec![2.5, 3.125, 5., 8.125, 12.5],
        vec![0.625, 1.25, 3.125, 6.25, 10.625],
        vec![0.0, 0.625, 2.5, 5.625, 10.0],
    ];
    let trace = Contour::new_z(z).color_scale(ColorScale::Palette(ColorScalePalette::Jet));

    let layout = Layout::new().title(Title::new("Colorscale for Contour Plot"));
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("colorscale_for_contour_plot"))
    );
}

fn customizing_size_and_range_of_a_contour_plots_contours(show: bool) {
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
        .contours(Contours::new().start(0.0).end(8.0).size(2));

    let layout = Layout::new().title(Title::new("Customizing Size and Range of Contours"));
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "customizing_size_and_range_of_a_contour_plots_contours"
        ))
    );
}

fn customizing_spacing_between_x_and_y_ticks(show: bool) {
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

    let layout = Layout::new().title(Title::new("Customizing Size and Range of Contours"));
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("customizing_spacing_between_x_and_y_ticks"))
    );
}

// Heatmaps
fn basic_heat_map(show: bool) {
    let z = vec![vec![1, 20, 30], vec![20, 1, 60], vec![30, 60, 1]];
    let trace = HeatMap::new_z(z);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("basic_heat_map")));
}

fn main() -> std::io::Result<()> {
    // Contour Plots
    simple_contour_plot(true);
    colorscale_for_contour_plot(true);
    customizing_size_and_range_of_a_contour_plots_contours(true);
    customizing_spacing_between_x_and_y_ticks(true);

    // Heatmaps
    basic_heat_map(true);
    Ok(())
}
