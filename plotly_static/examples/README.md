# Plotly Static Export CLI Example

This example demonstrates how to use the `plotly_static` crate with `clap` to create a command-line tool for exporting Plotly plots to static images.

## Usage

### Basic Usage

Export a plot from a JSON file (using Chrome driver):
```bash
cargo run --example main --features chromedriver -- -i sample_plot.json -o my_plot -f png
```

Export a plot from a JSON file (using Firefox/Gecko driver):
```bash
cargo run --example main --features geckodriver -- -i sample_plot.json -o my_plot -f png
```

Export a plot from stdin:
```bash
cat sample_plot.json | cargo run --example main --features chromedriver -- -f svg -o output
```

### Web Driver Options

The example supports two different web drivers for rendering plots:

- **Chrome Driver** (`--features chromedriver`): Uses Chrome/Chromium browser for rendering
- **Gecko Driver** (`--features geckodriver`): Uses Firefox browser for rendering

You must specify one of these features when running the example. For example:
```bash
# Use Chrome driver
cargo run --example main --features chromedriver -- -i plot.json -o output.png

# Use Firefox driver  
cargo run --example main --features geckodriver -- -i plot.json -o output.png
```

### Logging

The example uses `env_logger` for logging. You can enable different log levels using the `RUST_LOG` environment variable:

```bash
# Enable info level logging
RUST_LOG=info cargo run --example main --features chromedriver -- -i sample_plot.json -o my_plot -f png

# Enable debug level logging for more verbose output
RUST_LOG=debug cargo run --example main --features geckodriver -- -i sample_plot.json -o my_plot -f png

# Enable all logging levels
RUST_LOG=trace cargo run --example main --features chromedriver -- -i sample_plot.json -o my_plot -f png
```

### Command Line Options

- `-i, --input`: Input file containing Plotly JSON (use '-' for stdin, default: "-")
- `-o, --output`: Output file path (default: "output")
- `-f, --format`: Image format (png, jpeg, webp, svg, pdf, default: png)
- `--width`: Image width in pixels (default: 800)
- `--height`: Image height in pixels (default: 600)
- `-s, --scale`: Image scale factor (default: 1.0)
- `--offline`: Use offline mode (bundled JavaScript)

### Examples

Export to PNG with custom dimensions:
```bash
cargo run --example main --features chromedriver -- -i sample_plot.json -o plot -f png --width 1200 --height 800
```

Export to SVG from stdin:
```bash
echo '{"data":[{"type":"scatter","x":[1,2,3],"y":[4,5,6]}],"layout":{}}' | \
cargo run --example main --features geckodriver -- -f svg -o scatter_plot
```

Export to PDF with high resolution:
```bash
cargo run --example main --features chromedriver -- -i sample_plot.json -o report -f pdf --width 1600 --height 1200 -s 2.0
```

### JSON Format

The input JSON should follow the Plotly figure specification:

```json
{
  "data": [
    {
      "type": "surface",
      "x": [1.0, 2.0, 3.0],
      "y": [4.0, 5.0, 6.0],
      "z": [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]
    }
  ],
  "layout": {},
  "config": {}
}
```

## Features

- Support for all major image formats (PNG, JPEG, WEBP, SVG, PDF)
- Input from files or stdin
- Customizable dimensions and scale
- Offline mode support
- Comprehensive error handling
- Built-in help and version information
- Configurable logging with environment variables 