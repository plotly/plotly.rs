use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::{Bar, NamedColor, Plot, Rgb, Rgba, Scatter, Scatter3D, ScatterPolar};
use rand_distr::{Distribution, Normal, Uniform};

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
    println!("{}", plot.to_inline_html(Some("simple_scatter_plot")));
}

fn main() -> std::io::Result<()> {
    // Scatter3D Plots
    simple_scatter3d_plot(true);
    Ok(())
}
