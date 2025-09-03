use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use log::info;
use plotly_static::{ImageFormat, StaticExporterBuilder};
use serde_json::Value;

#[derive(Parser)]
#[command(name = "plotly-static-export")]
#[command(about = "Export Plotly plots to static images")]
#[command(version)]
struct Cli {
    /// Input file containing Plotly JSON (use '-' for stdin)
    #[arg(short, long, required = true, default_value = "-")]
    input: String,

    /// Output file path
    #[arg(short, long, default_value = "output")]
    output: PathBuf,

    /// Image format
    #[arg(short, long, value_enum, default_value_t = ImageFormatArg::PNG)]
    format: ImageFormatArg,

    /// Image width in pixels
    #[arg(long, default_value_t = 800)]
    width: usize,

    /// Image height in pixels
    #[arg(long, default_value_t = 600)]
    height: usize,

    /// Image scale factor
    #[arg(short, long, default_value_t = 1.0)]
    scale: f64,

    /// Use offline mode (bundled JavaScript)
    #[arg(long)]
    offline: bool,
}

#[derive(ValueEnum, Clone)]
#[allow(clippy::upper_case_acronyms)]
enum ImageFormatArg {
    PNG,
    JPEG,
    WEBP,
    SVG,
    PDF,
}

impl std::fmt::Display for ImageFormatArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageFormatArg::PNG => "PNG",
                ImageFormatArg::JPEG => "JPEG",
                ImageFormatArg::WEBP => "WEBP",
                ImageFormatArg::SVG => "SVG",
                ImageFormatArg::PDF => "PDF",
            }
        )
    }
}

impl From<ImageFormatArg> for ImageFormat {
    fn from(format: ImageFormatArg) -> Self {
        match format {
            ImageFormatArg::PNG => ImageFormat::PNG,
            ImageFormatArg::JPEG => ImageFormat::JPEG,
            ImageFormatArg::WEBP => ImageFormat::WEBP,
            ImageFormatArg::SVG => ImageFormat::SVG,
            ImageFormatArg::PDF => ImageFormat::PDF,
        }
    }
}

fn read_json_from_stdin() -> Result<Value, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let json: Value = serde_json::from_str(&buffer)?;
    Ok(json)
}

fn read_json_from_file(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&content)?;
    Ok(json)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    // Read JSON input
    let plot_json = if cli.input == "-" {
        info!("Reading Plotly JSON from stdin...");
        read_json_from_stdin()?
    } else {
        info!("Reading Plotly JSON from file: {}", cli.input);
        read_json_from_file(&cli.input)?
    };

    // Validate that the JSON has the expected structure
    if !plot_json.is_object() {
        return Err("Invalid JSON: expected an object".into());
    }

    // Build StaticExporter instance
    let mut exporter = StaticExporterBuilder::default()
        .offline_mode(cli.offline)
        .build()?;

    info!(
        "Exporting plot to {} format ({}x{} pixels, scale: {})...",
        cli.format, cli.width, cli.height, cli.scale
    );

    // Export the plot
    exporter.write_fig(
        &cli.output,
        &plot_json,
        cli.format.into(),
        cli.width,
        cli.height,
        cli.scale,
    )?;

    info!("Successfully exported plot to: {}", cli.output.display());
    Ok(())
}
