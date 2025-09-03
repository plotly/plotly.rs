# Static Export Example

This example demonstrates how to use the `plotly_static` crate for exporting Plotly plots to static images.

The `plotly_static` provides a interface for converting Plotly plots into various static image formats (PNG, JPEG, WEBP, SVG, PDF) using WebDriver and headless browsers.

In this example it is shown how to use the `StaticExporter` with the old style Kaleido API and also with the new style API. Using the former API is fine for one time static exports, but that API will crate an instance of the `StaticExporter` for each `write_image` call. The new style API is recommended for performance as the same instance of the `StaticExporter` can be reused across multiple exports.  

See also the `Static Image Export` section in the book for a more detailed description.

## Overview


## Features 

- **Multiple Export Formats**: PNG, JPEG, SVG, PDF
- **Exporter Reuse (new API)**: Efficient reuse of a single `StaticExporter` instance
- **String Export**: Base64 and SVG string output for web applications
- **Logging**: Debug information and progress monitoring
- **Error Handling**: Error handling with `Result` types

## Prerequisites

1. **Browser Installation**: You need either Firefox (for geckodriver) or Chrome/Chromium (for chromedriver)
2. **WebDriver**: Chromedriver automatically downloaded with the `static_export_default` feature
3. **Internet Connection**: Required for WebDriver download (if using `webdriver_download` feature)

## Feature Flags

The example uses `static_export_default` which includes:
- `plotly_static`: Core static export functionality
- `plotly_static/chromedriver`: Chrome WebDriver support
- `plotly_static/webdriver_download`: Automatic WebDriver download

### Alternative Configurations

```toml
# Use Firefox instead of Chrome/Chromium 
plotly = { version = "0.13", features = ["static_export_geckodriver", "static_export_wd_download"] }

# Manual Geckodriver installation (no automatic download)
plotly = { version = "0.13", features = ["static_export_geckodriver"] }

# Manual Chromedriver installation (no automatic download)
plotly = { version = "0.13", features = ["static_export_chromedriver"] }
```

## Running the Example

```bash
# Basic run
cargo run

# With debug logging
RUST_LOG=debug cargo run

# With custom WebDriver path
WEBDRIVER_PATH=/path/to/chromedriver cargo run
```

## Output

The example generates several files:
- `plot1.png`: Raster format, good for web/screens
- `plot2.jpeg`: Compressed raster, smaller file size
- `plot3.svg`: Vector format, scalable, good for web
- `plot1.pdf`: Vector format, good for printing

## Advanced Configuration

The example includes commented code showing advanced configuration options:
- Custom WebDriver ports for parallel usage
- Offline mode for bundled JavaScript 
- Custom browser capabilities
- Explicit WebDriver spawning

## Troubleshooting

### Common Issues

1. **WebDriver not found**: Ensure the browser is installed and WebDriver is available
2. **Port conflicts**: Use unique ports for parallel operations

### Debug Information

Set `RUST_LOG=debug` or`RUST_LOG=trace`to see detailed WebDriver operations and troubleshooting information.

## Related Documentation

- [plotly_static crate documentation](https://docs.rs/plotly_static/)
- [WebDriver specification](https://w3c.github.io/webdriver/)
- [GeckoDriver documentation](https://firefox-source-docs.mozilla.org/testing/geckodriver/)
- [ChromeDriver documentation](https://chromedriver.chromium.org/) 
