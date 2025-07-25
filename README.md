<div align="center">
  <h1>Plotly.rs</h1>
  <p><strong>Plotly for Rust</strong></p>
  <p>
    <a href="https://github.com/plotly/plotly.rs/actions?query=branch%3Amain" style="text-decoration: none!important;">
        <img src="https://img.shields.io/github/actions/workflow/status/plotly/plotly.rs/build.yml?branch=main" alt="Build status">
    </a>
    <a href="https://crates.io/crates/plotly" style="text-decoration: none!important;">
        <img src="https://img.shields.io/crates/v/plotly.svg" alt="Crates.io">
    </a>
    <a href="https://crates.io/crates/plotly" style="text-decoration: none!important;">
        <img src="https://img.shields.io/crates/d/plotly" alt="Downloads">
    </a>
	<a href="https://docs.rs/plotly" style="text-decoration: none!important;">
        <img src="https://img.shields.io/badge/docs.rs-plotly-green" alt="Documentation">
    </a>
	<a href="https://app.codecov.io/github/plotly/plotly.rs" style="text-decoration: none!important;">
        <img src="https://img.shields.io/codecov/c/github/igiagkiozis/plotly" alt="Code coverage">
    </a>
  </p>
  <h4>
    <a href="https://plotly.github.io/plotly.rs/content/getting_started.html">Getting Started</a>
    <span> | </span>
    <a href="https://plotly.github.io/plotly.rs/content/recipes.html">Recipes</a>
    <span> | </span>
    <a href="https://docs.rs/crate/plotly/">API Docs</a>
    <span> | </span>
    <a href="https://github.com/plotly/plotly.rs/tree/main/CHANGELOG.md">Changelog</a>
  </h4>
</div>

<div align="center">
  <a href="https://dash.plotly.com/project-maintenance">
    <img src="https://dash.plotly.com/assets/images/maintained-by-community.png" width="400px" alt="Maintained by the Plotly Community">
  </a>
</div>

# Table of Contents

* [Introduction](#introduction)
* [Basic Usage](#basic-usage)
    * [Exporting a single Interactive Plot](#exporting-a-single-interactive-plot)
    * [Exporting Static Images with plotly_static (Recommended)](#exporting-static-images-with-plotly_static-recommended)
    * [Exporting Static Images with Kaleido (legacy)](#exporting-static-images-with-kaleido-legacy)
    * [Usage Within a WASM Environment](#usage-within-a-wasm-environment)
* [Crate Feature Flags](#crate-feature-flags)
* [Contributing](#contributing)
* [Code of Conduct](#code-of-conduct)
* [License](#license)

# Introduction

A plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).

Documentation and numerous interactive examples are available in the [Plotly.rs Book](https://plotly.github.io/plotly.rs/content/getting_started.html), the [examples/](https://github.com/plotly/plotly.rs/tree/main/examples) directory and [docs.rs](https://docs.rs/crate/plotly).

For changes since the last version, please consult the [changelog](https://github.com/plotly/plotly.rs/tree/main/CHANGELOG.md).

# Basic Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
plotly = "0.13"
```

## Exporting a single Interactive Plot

Any figure can be saved as an HTML file using the `Plot.write_html()` method. These HTML files can be opened in any web browser to access the fully interactive figure.

```rust
use plotly::{Plot, Scatter};

let mut plot = Plot::new();
let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
plot.add_trace(trace);

plot.write_html("out.html");
```

By default, the Plotly JavaScript library and some [MathJax](https://docs.mathjax.org/en/latest/web/components/index.html) components will always be included via CDN, which results in smaller file-size, but slightly slower first load as the JavaScript libraries have to be downloaded first. Alternatively, to embed the JavaScript libraries (several megabytes in size) directly into the HTML file, `plotly-rs` must be compiled with the feature flag `plotly_embed_js`. With this feature flag the Plotly and MathJax JavaScript libraries are directly embedded in the generated HTML file. It is still possible to use the CDN version, by using the `use_cdn_js` method.

```rust
// <-- Create a `Plot` -->

plot.use_cdn_js();
plot.write_html("out.html");
```

If you only want to view the plot in the browser quickly, use the `Plot.show()` method.

```rust
// <-- Create a `Plot` -->

plot.show(); // The default web browser will open, displaying an interactive plot
```

## Exporting Static Images with plotly_static (Recommended)

The recommended way to export static images is using the `plotly_static` backend, which uses a headless browser via WebDriver (Chrome or Firefox) for rendering. This is available via the `static_export_default` feature:

```toml
[dependencies]
plotly = { version = "0.13", features = ["static_export_default"] }
```

This supports PNG, JPEG, WEBP, SVG, and PDF formats:

```rust
use plotly::{Plot, Scatter, ImageFormat};

let mut plot = Plot::new();
plot.add_trace(Scatter::new(vec![0, 1, 2], vec![2, 1, 0]));

plot.write_image("out.png", ImageFormat::PNG, 800, 600, 1.0)?;
plot.write_image("out.svg", ImageFormat::SVG, 800, 600, 1.0)?;
let base64_data = plot.to_base64(ImageFormat::PNG, 800, 600, 1.0)?;
let svg_string = plot.to_svg(800, 600, 1.0)?;
```

**Note:** This feature requires a WebDriver-compatible browser (Chrome or Firefox) as well as a Webdriver (chromedriver/geckodriver) to be available on the system. For advanced usage, see the [`plotly_static` crate documentation](https://docs.rs/plotly_static/).

## Exporting Static Images with Kaleido (legacy)

Enable the `kaleido` feature and opt in for automatic downloading of the `kaleido` binaries by doing the following

```toml
# Cargo.toml

[dependencies]
plotly = { version = "0.13", features = ["kaleido", "kaleido_download"] }
```

Alternatively, enable only the `kaleido` feature and manually install Kaleido.
```toml
# Cargo.toml

[dependencies]
plotly = { version = "0.13", features = ["kaleido"] }
```

With the feature enabled, plots can be saved as any of `png`, `jpeg`, `webp`, `svg`, `pdf` and `eps`. Note that the plot will be a static image, i.e. they will be non-interactive.

Exporting a simple plot looks like this:

```rust
use plotly::{ImageFormat, Plot};

let mut plot = Plot::new();
let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
plot.add_trace(trace);

plot.write_image("out.png", ImageFormat::PNG, 800, 600, 1.0);
```

### Kaleido external dependency

When developing applications for your host, enabling both `kaleido` and `kaleido_download` features will ensure that the `kaleido` binary is downloaded for your system's architecture at compile time. After download, it is unpacked into a specific path, e.g., on Linux this is `/home/USERNAME/.config/kaleido`. With these two features enabled, static images can be exported as described in the next section as long as the application runs on the same machine where it has been compiled on.

When the applications developed with `plotly.rs` are intended for other targets or when the user wants to control where the `kaleido` binary is installed then Kaleido must be manually downloaded and installed. Setting the environment variable `KALEIDO_PATH=/path/installed/kaleido/` will ensure that applications that were built with the `kaleido` feature enabled can locate the `kaleido` executable and use it to generate static images.

Kaleido binaries are available on Github [release page](https://github.com/plotly/Kaleido/releases). It currently supports Linux(`x86_64`), Windows(`x86_64`) and MacOS(`x86_64`/`aarch64`).

## Usage Within a WASM Environment

`Plotly.rs` can be used with a WASM-based frontend framework. The needed dependencies are automatically enabled for `wasm32` targets at compile time and there is no longer a need for the custom `wasm` flag in this crate. Note that the `kaleido` and `plotly_static` features are not supported in WASM environments and will throw a compilation error if enabled. 

First, make sure that you have the Plotly JavaScript library in your base HTML template:

```html
 <!-- index.html -->

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


    use_effect_with((), move |_| {
        p.run();
        || ()
    });


    html! {
        <div id="plot-div"></div>
    }
}
```

More detailed standalone examples can be found in the [examples/wasm-yew](https://github.com/plotly/plotly.rs/tree/main/examples/wasm-yew) directory.

# Crate Feature Flags

The following feature flags are available:

### `static_export_default`

Since version `0.13.0` support for exporting to static images is based on using a new crate called `plotly_static` that uses WebDriver and browser automation for static export functionality.

This feature flag automatically enables the usage of the `plotly_static` dependency as well as the `chromedriver` and `webdriver_download` features of that crate. For more details about these feature flags, refer to the `plotly_static` [documentation](plotly_static/README.md).   

The other related features allow controlling other aspects of the `plotly_static` crate
 - `static_export_chromedriver`
 - `static_export_geckodriver`
 - `static_export_wd_download`


### `plotly_image`

Adds trait implementations so that `image::RgbImage` and `image::RgbaImage` can be used more directly with the `plotly::Image` trace.

### `plotly_ndarray`

Adds support for creating plots directly using [ndarray](https://github.com/rust-ndarray/ndarray) types.

### `plotly_embed_js`

By default, the CDN version of `plotly.js` is used in the library and in the generated HTML files. This feature can be used to opt in for embedding `plotly.min.js` in the generated HTML files. The benefit is that the plot will load faster in the browser.

However, there are two downsides of using this feature flag, one is that the resulting html will be much larger, as a copy of the `plotly.min.js` library is embedded in each HTML file. The second, more relevant, is that a copy of the `plotly.min.js` library needs to be compiled in the `plotly-rs` library itself which increases the size by approx `3.5 Mb`.

When the feature is enabled, users can still opt in for the CDN version by using the method `use_cdn_js`.

Note that when using `Plot::to_inline_html()`, it is assumed that the `plotly.js` library is already in scope within the HTML file, so enabling this feature flag will have no effect.

### `kaleido` (legacy)

Adds plot save functionality to the following formats: `png`, `jpeg`, `webp`, `svg`, `pdf` and `eps`.

Requires `Kaleido` to have been previously installed on the host machine. See the following feature flag and [Kaleido external dependency](#kaleido-external-dependency).

### `kaleido_download` (legacy)

Enable download and install of Kaleido binary at build time from [Kaleido releases](https://github.com/plotly/Kaleido/releases/) on the host machine.
See [Kaleido external dependency](#kaleido-external-dependency) for more details.

# Contributing

* If you've spotted a bug or would like to see a new feature, please submit an issue on the [issue tracker](https://github.com/plotly/plotly.rs/issues).

* Pull requests are welcome, see the [contributing guide](https://github.com/plotly/plotly.rs/tree/main/CONTRIBUTING.md) for more information.

# Code of Conduct

See the [Code of Conduct](https://github.com/plotly/plotly.rs/tree/main/CODE_OF_CONDUCT.md) for more information.

# License

`Plotly.rs` is distributed under the terms of the MIT license, see [LICENSE](https://github.com/plotly/plotly.rs/tree/main/LICENSE).
