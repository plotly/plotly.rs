# plotly_static

Export Plotly plots to static images using WebDriver and headless browsers.

## Overview

`plotly_static` provides a Rust interface for converting Plotly plots from JSON data into various static image formats (PNG, JPEG, WEBP, SVG, PDF) using WebDriver and headless browsers.

## Features

- **Async/Sync API Support**: Support for both async and sync contexts
- **Multiple Formats**: PNG, JPEG, WEBP, SVG, PDF
- **Browser Support**: Chrome/Chromium (chromedriver) and Firefox (geckodriver)
- **Efficient**: Reuse `StaticExporter` instances for multiple exports
- **String Export**: Base64 and SVG output for web applications
- **Parallel Safe**: Designed for concurrent usage
- **Automatic Management**: Handles WebDriver lifecycle and cleanup

## Quick Start

```rust
use plotly_static::{StaticExporterBuilder, ImageFormat};
use serde_json::json;
use std::path::Path;

// Create a simple plot as JSON
let plot = json!({
    "data": [{
        "type": "scatter",
        "x": [1, 2, 3, 4],
        "y": [10, 11, 12, 13]
    }],
    "layout": {
        "title": "Simple Scatter Plot"
    }
});

// Build and use StaticExporter
let mut exporter = StaticExporterBuilder::default()
    .build()
    .expect("Failed to build StaticExporter");

// Export to PNG
exporter.write_fig(
    Path::new("my_plot"),
    &plot,
    ImageFormat::PNG,
    800,
    600,
    1.0
).expect("Failed to export plot");
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
plotly_static = { version = "0.1", features = ["chromedriver", "webdriver_download"] }
serde_json = "1.0"
```

### Feature Flags

- `chromedriver`: Use Chromedriver and Chrome/Chromium browser for rendering and export
- `geckodriver`: Use Geckodriver Firefox browser for rendering for rendering and export
- `webdriver_download`: Auto-download the chosen WebDriver binary

## Prerequisites

1. **Browser**: Chrome/Chromium or Firefox installed
2. **WebDriver**: Manually installed or automatically downloaded and installed with the `webdriver_download` feature
3. **Internet Connectivity**: Required for WebDriver download when using the auto-download and install feature

## Async Support

The library supports async operations. To use the async API you need to call `build_async` instead of `build` on the `StaticExporterBuilder` . This will
 return an `AsyncStaticExporter` instance where the `write_fig` and `write_to_string` methods are async.

 ```rust
 use plotly_static::StaticExporterBuilder;

 let exporter = StaticExporterBuilder::default()
     .build_async()
     .expect("Failed to build AsyncStaticExporter");
 ```

 Never use the `sync` API in `async` contexts. The `sync` API wraps the `async` API and uses a `tokio::runtime::Runtime` instance internally.  Using the `sync` API in an async context will cause runtime errors such as e.g., "Cannot drop a runtime in a context where blocking is not allowed. This happens when a runtime is dropped from within an
 asynchronous context." or similar ones.

## Advanced Usage

### Static Exporter Reuse

```rust
use plotly_static::{StaticExporterBuilder, ImageFormat};
use serde_json::json;
use std::path::Path;

let plot1 = json!({
    "data": [{"type": "scatter", "x": [1,2,3], "y": [4,5,6]}],
    "layout": {"title": "Plot 1"}
});

let plot2 = json!({
    "data": [{"type": "scatter", "x": [2,3,4], "y": [5,6,7]}],
    "layout": {"title": "Plot 2"}
});

let mut exporter = StaticExporterBuilder::default()
    .build()
    .expect("Failed to create StaticExporter");

// Reuse for multiple exports
exporter.write_fig(Path::new("plot1"), &plot1, ImageFormat::PNG, 800, 600, 1.0)?;
exporter.write_fig(Path::new("plot2"), &plot2, ImageFormat::JPEG, 800, 600, 1.0)?;
```

### String Export

```rust
use plotly_static::{StaticExporterBuilder, ImageFormat};
use serde_json::json;

let plot = json!({
    "data": [{"type": "scatter", "x": [1,2,3], "y": [4,5,6]}],
    "layout": {}
});

let mut exporter = StaticExporterBuilder::default()
    .build()
    .expect("Failed to create StaticExporter");

// Get base64 data for web embedding
let base64_data = exporter.write_to_string(&plot, ImageFormat::PNG, 400, 300, 1.0)?;

// Get SVG data (vector format)
let svg_data = exporter.write_to_string(&plot, ImageFormat::SVG, 400, 300, 1.0)?;
```

### Custom Configuration

```rust
use plotly_static::StaticExporterBuilder;

let mut exporter = StaticExporterBuilder::default()
    .webdriver_port(4445)  // Unique port for parallel usage
    .offline_mode(true)    // Use bundled JavaScript
    .webdriver_browser_caps(vec![
        "--headless".to_string(),
        "--no-sandbox".to_string(),
    ])
    .build()?;
```

## Environment Variables

- `WEBDRIVER_PATH`: Custom WebDriver binary location
- `BROWSER_PATH`: Custom browser binary location

## Examples

Check the self contatined examples in the examples folder. 

Similar examples are available in the [Plotly.rs package](https://github.com/plotly/plotly.rs), in [Plotly.rs Book](https://plotly.github.io/plotly.rs/) as well as the example in [Plotly.rs examples/static_export](https://github.com/plotly/plotly.rs/tree/main/examples/static_export).

## Documentation

- [API Documentation](https://docs.rs/plotly_static/)
- [Static Image Export Guide](https://github.com/plotly/plotly.rs/tree/main/docs/book/src/fundamentals/static_image_export.md)

## License

This package is licensed under the MIT License. 
