use log::info;
use plotly::plotly_static::{ImageFormat, StaticExporterBuilder};
use plotly::prelude::*;
use plotly::{Plot, Scatter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Create some plots
    let mut plot1 = Plot::new();
    plot1.add_trace(Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17]).name("trace1"));

    let mut plot2 = Plot::new();
    plot2.add_trace(Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9]).name("trace2"));

    std::fs::create_dir_all("./output").unwrap();

    info!("Creating AsyncStaticExporter with default configuration...");
    let mut exporter = StaticExporterBuilder::default()
        .webdriver_port(5111)
        .build_async()
        .expect("Failed to create AsyncStaticExporter");

    info!("Exporting multiple plots using a single AsyncStaticExporter...");
    exporter
        .write_image(
            &plot1,
            "./output/plot1_async_api",
            ImageFormat::PNG,
            800,
            600,
            1.0,
        )
        .await?;
    exporter
        .write_image(
            &plot1,
            "./output/plot1_async_api",
            ImageFormat::JPEG,
            800,
            600,
            1.0,
        )
        .await?;
    exporter
        .write_image(
            &plot2,
            "./output/plot2_async_api",
            ImageFormat::SVG,
            800,
            600,
            1.0,
        )
        .await?;
    exporter
        .write_image(
            &plot2,
            "./output/plot2_async_api",
            ImageFormat::PDF,
            800,
            600,
            1.0,
        )
        .await?;

    info!("Exporting to base64 and SVG strings with async API...");
    let _base64_data = exporter
        .to_base64(&plot1, ImageFormat::PNG, 400, 300, 1.0)
        .await?;
    let _svg_data = exporter.to_svg(&plot1, 400, 300, 1.0).await?;

    // Always close the exporter to ensure proper release of WebDriver resources
    exporter.close().await;

    info!("Async exports completed successfully!");
    Ok(())
}
