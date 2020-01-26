extern crate rand;
use plotly::Plot;
use plotly::charts::Scatter;
use plotly::charts::Trace;
use rand::Rng;


fn geometric_brownian_motion(s_0: f64, dt: f64, n: usize, drift: f64, diffusion: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Normal::new(0.0, 1.0);
    let mut v = Vec::<f64>::with_capacity(n);
    v.push(s_0);
    let drift_factor = 1.0 + drift * dt;
    let diffusion_factor = diffusion * dt.sqrt();
    for idx in 1..n {
        let rv = drift_factor + diffusion_factor * rng.sample(dist);
        let prod: f64 = rv * v[idx-1];
        v.push(prod);
    }
    v
}

fn main() -> std::io::Result<()> {
    let n = 2_000;
    let x = (0..n).collect();
    let y = geometric_brownian_motion(100.0, 1.0/365.0, n, 0.15, 0.5);
    let mut t = Scatter::new("some", x, y);
    t.showlegend = Some(true);
    t.name = "another".to_owned();

    let mut plot = Plot::new();
    plot.add_trace(Trace::Scatter(t));
    plot.show();

    Ok(())
}
