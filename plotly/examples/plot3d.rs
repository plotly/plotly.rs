use itertools_num::linspace;
use plotly::common::{ColorScale, ColorScalePalette, Marker, MarkerSymbol, Mode, Title};
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Scatter3D, Surface};

// 3D Scatter Plots
fn simple_scatter3d_plot(show: bool) {
    let n: usize = 100;
    let t: Vec<f64> = linspace(0., 10., n).collect();
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
    let z: Vec<f64> = t.iter().map(|x| x.cos()).collect();

    let trace = Scatter3D::new(t, y, z).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    if show {
        plot.show();
    }
}

fn customized_scatter3d_plot(show: bool) {
    let n: usize = 100;
    let t: Vec<f64> = linspace::<f64>(0., 10., n).collect();
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
    let z: Vec<f64> = t.iter().map(|x| x.cos()).collect();
    let sizelookup = z.clone();

    let trace = Scatter3D::new(t.clone(), y.clone(), z.iter().map(|i| -i))
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

    if show {
        plot.show();
    }
}

// 3D Line Plots
fn simple_line3d_plot(show: bool) {
    let n: usize = 100;
    let t: Vec<f64> = linspace(0., 10., n).collect();
    let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
    let z: Vec<f64> = t.iter().map(|x| x.cos()).collect();

    let trace = Scatter3D::new(t, y, z).mode(Mode::Lines);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    if show {
        plot.show();
    }
}

// 3D Surface Plot
fn surface_plot(show: bool) {
    let n: usize = 100;
    let x: Vec<f64> = linspace(-10., 10., n).collect();
    let y: Vec<f64> = linspace(-10., 10., n).collect();
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

    if show {
        plot.show();
    }
}

fn main() -> std::io::Result<()> {
    // Scatter3D Plots
    simple_scatter3d_plot(true);
    simple_line3d_plot(true);
    customized_scatter3d_plot(true);
    surface_plot(true);
    Ok(())
}
