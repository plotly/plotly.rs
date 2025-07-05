use log::info;
use plotly::plotly_static::{ImageFormat, StaticExporterBuilder};
use plotly::{Plot, Scatter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging for debugging and monitoring
    // Set RUST_LOG=debug to see detailed WebDriver operations
    env_logger::init();

    info!("Starting static export example...");
    info!("This example demonstrates efficient exporter reuse and multiple formats");

    // Create multiple plots for demonstration
    let mut plot1 = Plot::new();
    plot1.add_trace(Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17]).name("trace1"));

    let mut plot2 = Plot::new();
    plot2.add_trace(Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9]).name("trace2"));

    let mut plot3 = Plot::new();
    plot3.add_trace(Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12]).name("trace3"));

    // Create a single StaticExporter to reuse across all plots
    // This is more efficient than creating a new exporter for each plot
    info!("Creating StaticExporter with default configuration...");
    let mut exporter = StaticExporterBuilder::default()
        .build()
        .expect("Failed to create StaticExporter");

    info!("Exporting multiple plots using a single StaticExporter...");

    // Export all plots using the same exporter (more efficient)
    // Different formats demonstrate the variety of supported outputs
    plot1.write_image_with_exporter(&mut exporter, "plot1", ImageFormat::PNG, 800, 600, 1.0)?;
    plot2.write_image_with_exporter(&mut exporter, "plot2", ImageFormat::JPEG, 800, 600, 1.0)?;
    plot3.write_image_with_exporter(&mut exporter, "plot3", ImageFormat::SVG, 800, 600, 1.0)?;

    // Export to PDF (vector format, good for printing)
    plot1.write_image_with_exporter(&mut exporter, "plot1", ImageFormat::PDF, 800, 600, 1.0)?;

    // Demonstrate string-based export (useful for web applications)
    info!("Exporting to base64 and SVG strings...");

    // Get base64 data (useful for embedding in HTML or APIs)
    let base64_data =
        plot1.to_base64_with_exporter(&mut exporter, ImageFormat::PNG, 400, 300, 1.0)?;
    info!("Base64 data length: {}", base64_data.len());

    // Get SVG data (vector format, scalable, good for web)
    let svg_data = plot1.to_svg_with_exporter(&mut exporter, 400, 300, 1.0)?;
    info!("SVG data starts with: {}", &svg_data[..50]);

    info!("All exports completed successfully!");
    info!("Generated files:");
    info!("  - plot1.png (raster format, good for web/screens)");
    info!("  - plot2.jpeg (compressed raster, smaller file size)");
    info!("  - plot3.svg (vector format, scalable, good for web)");
    info!("  - plot1.pdf (vector format, good for printing)");

    // Demonstrate advanced configuration (commented out for this example)
    /*
    // For parallel usage or custom configuration:
    let mut custom_exporter = StaticExporterBuilder::default()
        .webdriver_port(4445)  // Use different port for parallel operations
        .spawn_webdriver(true) // Explicitly spawn WebDriver
        .offline_mode(true)    // Use bundled JavaScript (no internet required)
        .webdriver_browser_caps(vec![
            "--headless".to_string(),
            "--no-sandbox".to_string(),
            "--disable-gpu".to_string(),
            "--disable-dev-shm-usage".to_string(),
        ])
        .build()
        .expect("Failed to create custom StaticExporter");
    */

    Ok(())
}
