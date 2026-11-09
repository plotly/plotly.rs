# Proa page composition

This example adapts the three figures from
[`customization/multiple_plots_example`](../customization/multiple_plots_example)
into a page built with [Proa](https://proa.so). It demonstrates composing Plotly
figures as Rust render components, rather than inserting chart HTML strings
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

- `PlotLayout<C>` in `src/layout.rs` owns the HTML document, styles, shared
  Plotly.js script, and chart initialization. It accepts any synchronous child.
- The layout's `render_head` supplies defaults. The page overrides them with
  `HeadMetadata`, and `{cx.head()}` emits the final document metadata.
- `MultiplePlotsPage` in `src/page.rs` owns the page metadata and renders the
  page heading and a collection of `ChartCard` components.
- `ChartCard` supplies the common section/heading structure around `PlotlyChart`.
- `PlotlyChart::new` borrows a `Plot` and serializes it once during setup.
  Rendering borrows the prepared component and uses Proa's normal text escaping;
  there is no custom serializer or per-render JSON allocation. It does not call
  `to_html()` or `to_inline_html()` or clone the plot.

The plot-building functions still use the normal Plotly.rs API. They return
`Plot`, leaving presentation to the components. Change the layout once to change
all pages that use it, or change `ChartCard` once to change every chart wrapper.

The example pins a public Proa Git revision for the new managed metadata API,
which is not in the released 0.3 crates. Cargo fetches it automatically; no local
Proa checkout is required. The Git dependencies will be replaced with crates.io
versions after Proa 0.4 is released. `render_page` uses Proa's normal response
finalizer to prepare layout/page head hooks before writing the standalone HTML
file.

## Plotly.js boundary

Each chart renders a container followed by an inert `<template>` holding JSON.
Proa escapes the JSON as ordinary HTML text, and the browser decodes those
entities when reading the template's text content. This preserves the figure
without interpreting strings in its data as HTML. Chart IDs are escaped HTML
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
