use plotly::charts::{Surface, Lighting, PlaneContours,
                     PlaneProject, SurfaceContours};
use plotly::charts::Layout;
use plotly::Plot;


fn spectral_surface_plot() {
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    let mut z: Vec<Vec<f64>> = Vec::new();
    let N = 100;

    for i in 0..N {
        x.push(i as f64 / N as f64);
        y.push(i as f64 / N as f64);
    }

    for xi in 0..N {
        let mut iz: Vec<f64> = Vec::new();
        for yi in 0..N {
            let xf = (xi as f64) / N as f64;
            let yf = (yi as f64) / N as f64;

            let cz = (2.0 * xf * std::f64::consts::PI).sin() * (4.0 * yf * std::f64::consts::PI).cos();
            iz.push(cz);
        }
        z.push(iz);
    }

    let trace = Surface::new(z).x(x).y(y).visible(true)
        .hide_surface(false).lighting(Lighting::new())
        .contours(SurfaceContours::new().z(PlaneContours::new()
            .show(true).use_colormap(true).project(PlaneProject::new().z(true))));
    let mut plot = Plot::new();
    plot.add_layout(Layout::new());
    plot.add_trace(trace);
    plot.show();
}


fn main() -> std::io::Result<()> {
    spectral_surface_plot();

    Ok(())
}