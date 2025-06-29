use plotly_static::{ImageFormat, StaticExporterBuilder};
use serde_json::json;

fn main() {
    // Set BROWSER_PATH to test the binary capability
    std::env::set_var("BROWSER_PATH", "/usr/bin/google-chrome");

    let plot = json!({
        "data": [{
            "type": "scatter",
            "x": [1, 2, 3],
            "y": [4, 5, 6]
        }],
        "layout": {}
    });

    let mut exporter = StaticExporterBuilder::default()
        .build()
        .expect("Failed to build StaticExporter");

    // Test that we can export with the custom Chrome binary
    let svg_data = exporter
        .write_to_string(&plot, ImageFormat::SVG, 800, 600, 1.0)
        .expect("Failed to export plot");

    println!("Successfully exported SVG with custom Chrome binary!");
    println!("SVG length: {} characters", svg_data.len());
}
