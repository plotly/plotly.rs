# HTML Customization

We often get issues/questions regarding customization of the HTML output. In most situations, these are not related to Plotly functionality but rather custom behavior related to HTML rendering.

This directory contains examples of the most frequent raised questions by users of `plotly-rs`, such as:
- making the resulting HTML plot responsive on browser window size change
- making the resulting HTML fill the entire browser page
- placing multiple plots in the same HTML page, e.g., by using the [`build_html`](https://crates.io/crates/build_html) crate
- exporting plots to different formats with consistent font rendering

## Examples

### Density Mapbox Example
Demonstrates creating a responsive density mapbox plot with OpenStreetMap styling and zoom controls.

```bash
cd density_mapbox_example
cargo run
```

### Multiple Plots Example
Shows how to embed multiple plotly plots on a single HTML page using inline HTML generation.

```bash
cd multiple_plots_example
cargo run
```

### Consistent Format Export Example
Demonstrates exporting plots to SVG, PNG, and PDF formats with consistent font rendering across browsers.

```bash
cd consistent_static_format_export
cargo run
```
