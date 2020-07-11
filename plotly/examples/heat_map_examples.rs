use plotly::{HeatMap, Plot};

fn basic_heat_map() {
    let z = vec![vec![1, 20, 30], vec![20, 1, 60], vec![30, 60, 1]];
    let trace = HeatMap::new_z(z);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
}

fn main() -> std::io::Result<()> {
    basic_heat_map();
    Ok(())
}
