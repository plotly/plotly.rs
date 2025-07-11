# SVG Export Example

This example demonstrates exporting a plot to SVG, PNG, and PDF using plotly.rs and keeping font size and style consistent across SVG and other formats. 

This example is based on [GitHub Issue #171](https://github.com/plotly/plotly.rs/issues/171).

For consistent font rendering across browsers and export formats, always set the `font.family` property explicitly in your plot configuration. Relying on default or generic font settings can lead to differences in appearance, especially for font size and legend layout, depending on the browser or export backend.

The issue reported in [#171](https://github.com/plotly/plotly.rs/issues/171)is solved when the `font.family` property is set explicitly (e.g., to `"Times New Roman, serif"`) for all text elements (titles, axes, legends) which ensures consistent results in all output formats across different browsers.

## Running the Example

```bash
cd examples/customization/svg_export_example
cargo run
```

This will generate three output files:
- `Data_plot.png` 
- `Data_plot.svg` - will render fonts differently when compared with PNG if font family not fully specified
- `Data_plot.pdf` - uses SVG for PDF generation and will inherit the same issue as SVG
