# Proa page composition

This example adapts the three figures from
[`customization/multiple_plots_example`](../customization/multiple_plots_example)
into a page built with [Proa](https://proa.so). It demonstrates composing borrowed
`Plot` values as Rust render components, rather than inserting chart HTML strings
into a page builder.

## Run

Requires Rust 1.93 or newer. From this directory:

```bash
cargo run
```

Open `output/multiple_plots.html` in your browser. The file loads Plotly.js 3.7.0
from its CDN, so an internet connection is required. No application server, WASM
target, JavaScript build tool, or local Proa checkout is needed.

To run the render and serialization tests:

```bash
cargo test
```

## Components

- `PlotLayout<C>` owns the HTML document, styles, shared Plotly.js script, and
  initialization of all charts. It accepts any synchronous renderable child.
- `LayoutMetadata` supplies default title and description values; `PageMetadata`
  overrides them through Proa's metadata cascade. Other pages can inherit the
  defaults or supply their own metadata.
- `MultiplePlotsPage` renders the page heading and a collection of `ChartCard`
  components.
- `ChartCard` supplies the common section/heading structure around `PlotlyChart`.
- `PlotlyChart` is a small, example-local adapter over `&Plot`. It uses the
  existing `Serialize` implementation and writes JSON directly into Proa's output
  buffer. It does not call `to_html()` or `to_inline_html()` or clone the plot.

The plot-building functions still use the normal Plotly.rs API. They return
`Plot`, leaving presentation to the components. Change the layout once to change
all pages that use it, or change `ChartCard` once to change every chart wrapper.

This example uses the published Proa 0.3 metadata API (`Metadata` and
`resolve_metadata_stack`). It does not require the newer, unreleased
`render_head`/managed-head API.

## Plotly.js boundary

Each chart renders a container followed by an inert `<template>` holding JSON.
The serializer encodes HTML-sensitive characters as JSON escapes so the browser
preserves the data without interpreting it as HTML. Chart IDs are escaped HTML
attributes, never interpolated into JavaScript. The layout's initialization
script passes each container DOM node and parsed seed to `Plotly.newPlot`.

The short JavaScript bridge is limited to Plotly's third-party API, which has no
RSJS binding here. Plotly.js owns the interactive charts and window resizing;
Proa renders the page on the Rust side. There are no application filters, live
data updates, or independent component mounting/unmounting in this static-file
example.

The example does not replace Plotly.rs's internal Askama templates, modify its
public API, or change its default dependencies. Plotly.js and the figure data
still go to the browser. No rendering-speed or client-payload reduction is
claimed.
