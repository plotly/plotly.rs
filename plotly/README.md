<h1 align="center">Plotly.<span></span>rs</h1>

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

<div align="center">
    <a href="https://github.com/igiagkiozis/plotly/actions">
        <img src="https://github.com/igiagkiozis/plotly/workflows/build_master/badge.svg" alt="build status">
    </a>
    <a href="https://crates.io/crates/plotly">
        <img src="https://img.shields.io/crates/v/plotly.svg" alt="Crates.io">
    </a>
    <a href="https://crates.io/crates/plotly">
        <img src="https://img.shields.io/crates/d/plotly" alt="Downloads">
    </a>
	<a href="https://docs.rs/plotly">
        <img src="https://docs.rs/plotly/badge.svg" alt="Documentation">
    </a>
    <a href="">
        <img src="https://img.shields.io/badge/Minimum%20Rust%20Version-1.31-brightgreen.svg" alt="Minimum Version">
    </a>
</div>

A plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
plotly = "0.8.1"
```

For changes since the last version please consult the [change log](https://github.com/igiagkiozis/plotly/blob/master/CHANGELOG.md).

## Crate Feature Flags
The following feature flags are available:
* `kaleido`
    * Optional, compatible with Rust stable.
    * Adds plot save functionality to the following formats: png, jpeg, webp, svg, pdf and eps.
    * Requires some additional configuration, see [plotly_kaleido](https://github.com/igiagkiozis/plotly/tree/master/plotly_kaleido).
* `plotly_ndarray`
    * Optional, compatible with Rust stable.
    * Adds support for creating plots directly using [ndarray](https://github.com/rust-ndarray/ndarray) types.
* `wasm`
    * Optional, compatible with Rust stable.
    * Adds support for building with wasm-unknown-unknown target triple, enabling use within web development.

Saving to png, jpeg, webp, svg, pdf and eps formats can be made available by enabling the `kaleido` feature: 

```toml
[dependencies]
plotly = { version = "0.8.1", features = ["kaleido"] }
```
For further details please see [plotly_kaleido](https://github.com/igiagkiozis/plotly/tree/master/plotly_kaleido).


# Contributing

Please consult the [contributing guide](https://github.com/igiagkiozis/plotly/blob/master/CONTRIBUTING.md).

# License

Plotly.rs is distributed under the terms of the MIT license.

See [LICENSE-MIT](https://github.com/igiagkiozis/plotly/blob/master/LICENSE-MIT), and [COPYRIGHT](https://github.com/igiagkiozis/plotly/blob/master/COPYRIGHT) for details.
