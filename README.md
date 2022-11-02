<h1 align="center">Plotly.rs</h1>

<div align="center">Plotly for Rust</div>

<div align="center">
	<a href="https://igiagkiozis.github.io/plotly/content/getting_started.html">Getting Started</a>
    |
    <a href="https://igiagkiozis.github.io/plotly/content/recipes.html">Recipes</a>
    |
    <a href="https://docs.rs/crate/plotly/">API Docs</a>
    |
    <a href="https://github.com/igiagkiozis/plotly/blob/master/CHANGELOG.md">Changelog</a>
</div>
<br>
<div align="center">
    <a href="https://github.com/igiagkiozis/plotly/actions" style="text-decoration: none !important;">
        <img src="https://github.com/igiagkiozis/plotly/workflows/build_master/badge.svg" alt="build status">
    </a>
    <a href="https://crates.io/crates/plotly" style="text-decoration: none !important;">
        <img src="https://img.shields.io/crates/v/plotly.svg" alt="Crates.io">
    </a>
    <a href="https://crates.io/crates/plotly" style="text-decoration: none !important;">
        <img src="https://img.shields.io/crates/d/plotly" alt="Downloads">
    </a>
	<a href="https://docs.rs/plotly" style="text-decoration: none !important;">
        <img src="https://docs.rs/plotly/badge.svg" alt="Documentation">
    </a>
    <a href="" style="text-decoration: none !important;">
        <img src="https://img.shields.io/badge/Minimum%20Rust%20Version-1.31-brightgreen.svg" alt="Minimum Version">
    </a>
</div>
<br>

# Table of Contents

* [Introduction](#introduction)
* [Basic Usage](#basic-usage)
    * [Exporting an Interactive Plot](#exporting-an-interactive-plot)
    * [Exporting a Static Image](#exporting-a-static-image)
    * [Usage Within a Wasm Environment](#usage-within-a-wasm-environment)
* [Crate Feature Flags](#crate-feature-flags)
* [Contributing](#contributing)
* [License](#license)

# Introduction

A plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).

Documentation and numerous interactive examples are available in the [Plotly.rs Book](https://igiagkiozis.github.io/plotly/content/getting_started.html), the [examples/](https://github.com/igiagkiozis/plotly/tree/master/plotly/examples) directory and [docs.rs](https://docs.rs/crate/plotly).


For changes since the last version, please consult the [changelog](https://github.com/igiagkiozis/plotly/blob/master/CHANGELOG.md).

# Basic Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
plotly = "0.8.1"
```

## Exporting an Interactive Plot

Any figure can be saved as an HTML file using the `Plot.write_html()` method. These HTML files can be opened in any web browser to access the fully interactive figure.

```rust
use plotly::{Plot, Scatter};

let mut plot = Plot::new();
let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
plot.add_trace(trace);

plot.write_html("out.html");
```

By default, the Plotly JavaScript library will be included via CDN, which results in a smaller filesize, but slightly slower first load as the JavaScript library has to be downloaded first. To instead embed the JavaScript library (several megabytes in size) directly into the HTML file, the following can be done:

```rust
// <-- Create a `Plot` -->

plot.use_local_plotly();
plot.write_html("out.html");
```

If you only want to view the plot in the browser quickly, use the `Plot.show()` method.

```rust
// <-- Create a `Plot` -->

plot.show(); // The default web browser will open, displaying an interactive plot
```

## Exporting a Static Image

To save a plot as a static image, the `kaleido` feature is required:

```toml
# Cargo.toml

[dependencies]
plotly = { version = "0.8.1", features = ["kaleido"] }
```

With this feature enabled, plots can be saved as any of `png`, `jpeg`, `webp`, `svg`, `pdf` and `eps`. Note that the plot will be a static image, i.e. they will be non-interactive.

The Kaleido binary is downloaded for your system's architecture at compile time from the official Kaleido [release page](https://github.com/plotly/Kaleido/releases). This library currently supports `x86_64` on Linux and Windows, and both `x86_64` and `aarch64` on macOS.

Exporting a simple plot looks like this:

```rust
use plotly::{ImageFormat, Plot};

let mut plot = Plot::new();
let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
plot.add_trace(trace);

plot.write_image("out.png", ImageFormat::PNG, 800, 600, 1.0);
```

## Usage Within a Wasm Environment

Using `Plotly.rs` in a Wasm-based frontend framework is possible by enabling the `wasm` feature:

```toml
// Cargo.toml

[dependencies]
plotly = { version = "0.8.1", features = ["wasm"] }
```

First, make sure that you have the Plotly JavaScript library in your base HTML template:

```html
// index.html

<!doctype html>
<html lang="en">
    <head>
        <!-- snip -->
        <script src="https://cdn.plot.ly/plotly-2.14.0.min.js"></script>
    </head>
    <!-- snip -->
</html>
```

A simple `Plot` component would look as follows, using `Yew` as an example frontend framework:

```rust
use plotly::{Plot, Scatter};
use yew::prelude::*;


#[function_component(PlotComponent)]
pub fn plot_component() -> Html {
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";
        let mut plot = Plot::new();
        let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
        plot.add_trace(trace);

        async move {
            plotly::bindings::new_plot(id, &plot).await;
            Ok(())
        }
    });

    
        use_effect_with_deps(move |_| {
            p.run();
            || ()
        }, (),
    );
    

    html! {
        <div id="plot-div"></div>
    }
}
```

More detailed standalone examples can be found in the [examples/](https://github.com/igiagkiozis/plotly/tree/master/plotly/examples) directory.

# Crate Feature Flags

The following feature flags are available:

### `kaleido`

Adds plot save functionality to the following formats: `png`, `jpeg`, `webp`, `svg`, `pdf` and `eps`.

### `plotly_ndarray`

Adds support for creating plots directly using [ndarray](https://github.com/rust-ndarray/ndarray) types.

### `wasm`

Disables OS-specific functions, therefore allowing compilation in WASM environments.

# Contributing

* If you've spotted a bug or would like to see a new feature, please submit an issue on the [issue tracker](https://github.com/igiagkiozis/plotly/issues).

* Pull requests are welcome, see the [contributing guide](https://github.com/igiagkiozis/plotly/blob/master/CONTRIBUTING.md) for more information.

# License

`Plotly.rs` is distributed under the terms of the MIT license.

See [LICENSE-MIT](https://github.com/igiagkiozis/plotly/blob/master/LICENSE-MIT), and [COPYRIGHT](https://github.com/igiagkiozis/plotly/blob/master/COPYRIGHT) for details.