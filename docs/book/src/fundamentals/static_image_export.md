# Static Image Export

The `plotly` crate provides static image export functionality through the `plotly_static` crate, which uses WebDriver and headless browsers to render plots as static images.

## Overview

Static image export allows you to convert Plotly plots into various image formats (PNG, JPEG, WEBP, SVG, PDF) for use in reports, web applications, or any scenario where you need static images.

## Feature Flags

The static export functionality is controlled by feature flags in the main `plotly` crate:

### Required Features (choose one):
- `static_export_chromedriver`: Uses Chrome/Chromium for rendering (requires chromedriver)
- `static_export_geckodriver`: Uses Firefox for rendering (requires geckodriver)

### Optional Features:
- `static_export_wd_download`: Automatically downloads WebDriver binaries at build time
- `static_export_default`: Convenience feature that includes chromedriver + downloader

### Cargo.toml Configuration Examples:

```toml
# Basic usage with manual Chromedriver installation
[dependencies]
plotly = { version = "0.13", features = ["static_export_chromedriver"] }

# With automatic Chromedriver download
[dependencies]
plotly = { version = "0.13", features = ["static_export_chromedriver", "static_export_wd_download"] }

# Recommended: Default configuration with Chromedriver + auto-download
[dependencies]
plotly = { version = "0.13", features = ["static_export_default"] }
```

## Prerequisites

1. **WebDriver Installation**: You need either chromedriver or geckodriver installed
   - Chrome: Download from https://chromedriver.chromium.org/
   - Firefox: Download from https://github.com/mozilla/geckodriver/releases
   - Or use the `static_export_wd_download` feature for automatic download

2. **Browser Installation**: You need Chrome/Chromium or Firefox installed

3. **Environment Variables** (optional): 
   - Set `WEBDRIVER_PATH` to specify custom WebDriver binary location (should point to the full executable path)
   - Set `BROWSER_PATH` to specify custom browser binary location (should point to the full executable path)
   
   ```bash
   export WEBDRIVER_PATH=/path/to/chromedriver
   export BROWSER_PATH=/path/to/chrome
   ```

## Basic Usage

### Simple Export

```rust
use plotly::{Plot, Scatter, ImageFormat};

let mut plot = Plot::new();
plot.add_trace(Scatter::new(vec![1, 2, 3], vec![4, 5, 6]));

// Export to PNG file
plot.write_image("my_plot", ImageFormat::PNG, 800, 600, 1.0)
    .expect("Failed to export plot");
```

### Efficient Exporter Reuse

For better performance when exporting multiple plots, reuse a single `StaticExporter`:

```rust
use plotly::{Plot, Scatter};
use plotly::plotly_static::{StaticExporterBuilder, ImageFormat};

let mut plot1 = Plot::new();
plot1.add_trace(Scatter::new(vec![1, 2, 3], vec![4, 5, 6]));

let mut plot2 = Plot::new();
plot2.add_trace(Scatter::new(vec![2, 3, 4], vec![5, 6, 7]));

// Create a single exporter to reuse
let mut exporter = StaticExporterBuilder::default()
    .build()
    .expect("Failed to create StaticExporter");

// Export multiple plots using the same exporter
plot1.write_image_with_exporter(&mut exporter, "plot1", ImageFormat::PNG, 800, 600, 1.0)
    .expect("Failed to export plot1");
plot2.write_image_with_exporter(&mut exporter, "plot2", ImageFormat::JPEG, 800, 600, 1.0)
    .expect("Failed to export plot2");
```

## Supported Formats

### Raster Formats
- **PNG**: Portable Network Graphics, lossless compression
- **JPEG**: Joint Photographic Experts Group, lossy compression (smaller files)
- **WEBP**: Google's image format

### Vector Formats
- **SVG**: Scalable Vector Graphics
- **PDF**: Portable Document Format

### Deprecated
- **EPS**: Encapsulated PostScript (will be removed in version 0.14.0)

## String Export

For web applications or APIs, you can export to strings:

```rust
use plotly::{Plot, Scatter};
use plotly::plotly_static::{StaticExporterBuilder, ImageFormat};

let mut plot = Plot::new();
plot.add_trace(Scatter::new(vec![1, 2, 3], vec![4, 5, 6]));

let mut exporter = StaticExporterBuilder::default()
    .build()
    .expect("Failed to create StaticExporter");

// Get base64 data (useful for embedding in HTML)
let base64_data = plot.to_base64_with_exporter(&mut exporter, ImageFormat::PNG, 400, 300, 1.0)
    .expect("Failed to export plot");

// Get SVG data (vector format, scalable)
let svg_data = plot.to_svg_with_exporter(&mut exporter, 400, 300, 1.0)
    .expect("Failed to export plot");
```

## Advanced Configuration

### Custom WebDriver Configuration

```rust
use plotly::plotly_static::StaticExporterBuilder;

let mut exporter = StaticExporterBuilder::default()
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
    .expect("Failed to create StaticExporter");
```

### Parallel Usage

For parallel operations (tests, etc.), use unique ports:

```rust
use plotly::plotly_static::StaticExporterBuilder;
use std::sync::atomic::{AtomicU32, Ordering};

// Generate unique ports for parallel usage
static PORT_COUNTER: AtomicU32 = AtomicU32::new(4444);

fn get_unique_port() -> u32 {
    PORT_COUNTER.fetch_add(1, Ordering::SeqCst)
}

// Each thread/process should use a unique port
let mut exporter = StaticExporterBuilder::default()
    .webdriver_port(get_unique_port())
    .build()
    .expect("Failed to build StaticExporter");
```

## Logging Support

Enable logging for debugging and monitoring:

```rust
use plotly::plotly_static::StaticExporterBuilder;

// Initialize logging (typically done once at the start of your application)
env_logger::init();

// Set log level via environment variable
// RUST_LOG=debug cargo run

let mut exporter = StaticExporterBuilder::default()
    .build()
    .expect("Failed to create StaticExporter");
```

## Performance Considerations

- **Exporter Reuse**: Create a single `StaticExporter` and reuse it for multiple plots
- **Parallel Usage**: Use unique ports for parallel operations (tests, etc.)
- **Resource Management**: The exporter automatically manages WebDriver lifecycle

## Complete Example

See the [static export example](../../../examples/static_export/) for a complete working example that demonstrates:

- Multiple export formats
- Exporter reuse
- String export
- Logging
- Error handling

To run the example:

```bash
cd examples/static_export
cargo run
```
**NOTE** Set `RUST_LOG=debug` to see detailed WebDriver operations and troubleshooting information.

## Related Documentation

- [plotly_static crate documentation](https://docs.rs/plotly_static/)
- [WebDriver specification](https://w3c.github.io/webdriver/)
- [GeckoDriver documentation](https://firefox-source-docs.mozilla.org/testing/geckodriver/)
- [ChromeDriver documentation](https://chromedriver.chromium.org/)
