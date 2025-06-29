# SVG Export Example

This example demonstrates exporting a plot to SVG, PNG, and PDF using plotly.rs and keeping font size and style consistent across SVG and other formats. 

This example is based on [GitHub Issue #171](https://github.com/plotly/plotly.rs/issues/171).


**Summary:**

For consistent font rendering across browsers and export formats, always set the `font.family` property explicitly in your plot configuration. Relying on default or generic font settings can lead to differences in appearance, especially for font size and legend layout, depending on the browser or export backend.

**Recommendation:**

Always set the `font.family` property (e.g., to `"Times New Roman, serif"`) for all text elements (titles, axes, legends) to ensure consistent results in all output formats.

## Overview

This example creates a line and scatter plot with custom styling, including:
- Large font sizes for titles, legends, and axes
- Custom legend positioning and styling
- Border shapes around the plot
- Export to multiple formats (PDF, SVG, PNG)

## Running the Example

```bash
cd examples/customization/svg_export_example
cargo run
```

This will generate three output files:
- `Data_plot.pdf` - PDF format (typically renders correctly)
- `Data_plot.svg` - SVG format (may have font/legend issues)
- `Data_plot.png` - PNG format (typically renders correctly)
