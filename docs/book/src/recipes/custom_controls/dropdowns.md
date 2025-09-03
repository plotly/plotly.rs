# Dropdown Menus

Dropdown menus, implemented as an `UpdateMenu` in Plotly.rs, allow users to modify a plot's data or layout attributes by selecting an option from a list.

## Dropdown for Selecting Data

You can use a dropdown menu to switch between different sets of data. In this example, we show two different bar charts and use a dropdown to select which one is visible.

```rust,no_run
{{#include ../../../../../examples/custom_controls/src/main.rs:bar_plot_with_dropdown_for_different_data}}
```
{{#include ../../../../../examples/custom_controls/output/inline_bar_plot.html}}

## Dropdown for Modifying Layout

Update menus can also modify layout attributes. Here, we use buttons (a variation of a dropdown) to change the `bar_mode` of a bar chart, toggling between grouped and stacked bars.

```rust,no_run
{{#include ../../../../../examples/custom_controls/src/main.rs:bar_chart_with_modifiable_bar_mode}}
```
{{#include ../../../../../examples/custom_controls/output/inline_bar_chart.html}}

## Dropdown for Modifying Colorscales

You can dynamically change the colorscale of a heatmap or contour plot.

```rust,no_run
{{#include ../../../../../examples/custom_controls/src/main.rs:heat_map_with_modifiable_colorscale}}
```
{{#include ../../../../../examples/custom_controls/output/inline_heat_map.html}} 