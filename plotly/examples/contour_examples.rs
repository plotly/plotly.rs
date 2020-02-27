use plotly::{Plot, Contour, Layout};
use std::f64::consts::PI;
use plotly::common::{Title, ColorScale, ColorScalePalette};
use plotly::contour::Contours;

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

    for xi in 0..n {
        let mut row = Vec::<f64>::new();
        for yi in 0..n {
            let radius_squared = x[xi].powf(2.0) + y[yi].powf(2.0);
            let zv = x[xi].sin() * y[yi].cos() * radius_squared.sin() / (radius_squared + 1.0).log10();
            row.push(zv);
        }
        z.push(row);
    }

    let trace = Contour::new(x, y,z);
    let mut plot = Plot::new();

    plot.add_trace(trace);
    plot.show();
}

fn colorscale_for_contour_plot() {
    let z = vec![vec![10.0, 10.625, 12.5, 15.625, 20.0],
        vec![5.625, 6.25, 8.125, 11.25, 15.625],
        vec![2.5, 3.125, 5., 8.125, 12.5],
        vec![0.625, 1.25, 3.125, 6.25, 10.625],
        vec![0.0, 0.625, 2.5, 5.625, 10.0]];
    let trace = Contour::new_z(z).color_scale(ColorScale::Palette(ColorScalePalette::Jet));

    let layout = Layout::new().title(Title::new("Colorscale for Contour Plot"));
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace);
    plot.show();
}

fn customizing_size_and_range_of_a_contour_plots_contours() {
    let z = vec![vec![10.0, 10.625, 12.5, 15.625, 20.0],
                 vec![5.625, 6.25, 8.125, 11.25, 15.625],
                 vec![2.5, 3.125, 5., 8.125, 12.5],
                 vec![0.625, 1.25, 3.125, 6.25, 10.625],
                 vec![0.0, 0.625, 2.5, 5.625, 10.0]];
    let trace = Contour::new_z(z)
        .color_scale(ColorScale::Palette(ColorScalePalette::Jet))
        .auto_contour(false)
        .contours(Contours::new().start(0.0).end(8.0).size(2));

    let layout = Layout::new().title(Title::new("Customizing Size and Range of Contours"));
    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace);
    plot.show();
}

fn customizing_spacing_between_x_and_y_ticks() {
    let z = vec![vec![10.0, 10.625, 12.5, 15.625, 20.0],
                 vec![5.625, 6.25, 8.125, 11.25, 15.625],
                 vec![2.5, 3.125, 5., 8.125, 12.5],
                 vec![0.625, 1.25, 3.125, 6.25, 10.625],
                 vec![0.0, 0.625, 2.5, 5.625, 10.0]];
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
    plot.show();
}

fn main() -> std::io::Result<()> {
    simple_contour_plot();
    colorscale_for_contour_plot();
    customizing_size_and_range_of_a_contour_plots_contours();
    customizing_spacing_between_x_and_y_ticks();
    Ok(())
}
