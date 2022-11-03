# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.2] - 2022-11-03
### Added
- [[#110](https://github.com/igiagkiozis/plotly/pull/110)] `LegendGroupTitle` to existing traces.
- [[#88](https://github.com/igiagkiozis/plotly/pull/88)] `Mesh3D`, `Image`, `ScatterMapbox` traces.

### Changed
- [[#113](https://github.com/igiagkiozis/plotly/pull/113)] Refactored the structure of the examples to make them more accessible, whilst adding more examples e.g. for `wasm`.
- [[#115](https://github.com/igiagkiozis/plotly/pull/115)] Simplify the function signature of Plot.to_inline_html() so that it just takes `Option<&str>` as an argument.

## [0.8.1] - 2022-09-25
### Added
- Button support (i.e. [updatemenus](https://plotly.com/javascript/reference/layout/updatemenus/)) contibuted by [@sreenathkrishnan](https://github.com/sreenathkrishnan). Details and examples in this well written PR [#99](https://github.com/igiagkiozis/plotly/pull/99).
- Internally, there is now a `plotly-derive` crate which defines a `FieldSetter` procedural macro. This massively cuts down the amount of code duplication by generating the setter methods based on the struct fields. Again thanks to @sreenathkrishnan for this effort.

## [0.8.0] - 2022-08-26
Version 0.8.0 represents a significant release which refactors a lot of the codebase and tries to provide a cleaner API: there are several breaking changes listed below. On the whole, if migrating from v0.7.0, start by following any new compiler errors and, if you're still stuck, open an issue on the issue tracker and we can help out.
### Added
- impl `Clone`, `Serialize` and `PartialEq` for `Plot`
- impl `Clone` +/- `Copy` for color types
- Support for [configuration options](https://plotly.com/javascript/configuration-options/)
- Support for layout templates and pre-defined themes
- Support for WASM environments
- A much enhanced test suite
- Support for `Sankey` diagrams
- Support for `Plot3D` - 3D plots for scatter, line and surface data
### Changed
- Improve implementation of `private::NumOrString` to support more primitive types ([Issue 
#47](https://github.com/igiagkiozis/plotly/issues/47))
- Remove `private::TruthyEnum` in favour of a more robust way of serializing to `String` or `bool`
- Refactor `Color` module
- Refactored HTML templates with cleaner Javascript code
- `Plot::save()` is renamed to `Plot::write_image()`
- `Plot::show_{png | jpeg}` is now made generic via the new `Plot::show_image()` across static image formats with the `ImageFormat` enum
- `Plot::write_html()` now takes a filepath and saves the plot to that location
- `Plot::to_html()` now has similar behaviour to `Plot::to_inline_html()` and just returns a `String`
### Fixed
- Typos in `CONTRIBUTING.md`
- Serialization of `plotly_kaleido::PlotData` ([Issue #50](https://github.com/igiagkiozis/plotly/issues/50))
### Updated
- `ndarray` to `0.15.4`.
- `serde` to `1.0.132`.
- `serde_json` to `1.0.73`.
- `askama` to `0.11.0`.
- `rand` to `0.8`.
- `rand_distr` to `0.4`.
- `plotly.js` to `2.12.1`

## [0.7.0] - 2022-01-01
### Added
- `ScatterPolar` contributed by @samlich.
-  Add a method to render the plot to anything implementing Write, contributed by @foresterre.

### Changed
- Using plotly version 2.2.1
- Updated Kaleido version in `plotly_kaleido` to 0.2.1.
### Fixed
- Axis ticks text has the wrong parameter name (Issue #45).

## [0.6.0] - 2021-01-31
### Added
- tag matches for struct Axis : allow for synchronisation between subplots on x-axis
- fn matches in impl of Axis

## [0.6.0] - 2020-07-25
### Added
- Shapes support ([documentation](https://igiagkiozis.github.io/plotly/content/fundamentals/shapes.html)).
- Annotations support.
- Docstrings to `Scatter`.
- `ndarray` support ([documentation](https://igiagkiozis.github.io/plotly/content/fundamentals/ndarray_support.html)).
- Jupyter lab and notebook support ([documentation](https://igiagkiozis.github.io/plotly/content/fundamentals/jupyter_support.html)).
### Changed
- Removed `num` dependence.
- Removed `plotly_orca` and the `orca` feature. Use the `kaleido` feature for static image generation.
- Updated Kaleido version in `plotly_kaleido` to 0.0.1.


## [0.5.1] - 2020-07-12
### Added
- From<&str> implementation for `Title`.
### Changed
- Optimised dependencies.
- Clearer deprecation message for `orca` related methods.
- Wider use of `Default`.


## [0.5.0] - 2020-07-12
### Added
- [Plotly.rs Book](https://igiagkiozis.github.io/plotly/).
- Using plotly.js from the official CDN is now the default. To use the local version use the `Plot::use_local_plotly` method.
- Plot rasterization to `png`, `jpg`, `eps`, `pdf`, `webp` and `svg` using [plotly/Kaleido](https://github.com/plotly/Kaleido), enabled using the `kaleido` feature.
- Multi-axis support and examples.
- Subplot support and examples.
- Colors can now be floating values within the color-scale range.
- OpenGL support for scatter plots (`Scatter::open_gl_mode`); useful for displaying large data-sets.
### Changed
- `Layout` complete.
- Updated `plotly` version to `1.54.6`.
- `plotly_orca` and the `orca` feature are deprecated; use `kaleido` instead.
- All examples have been grouped to mirror the Plotly.rs book.


## [0.4.1] - 2020-03-26
### Fixed
- Added error message to capture the scenario when there is no default browser (or no browser at all) on a machine.
The message suggests a few alternatives.

## [0.4.0] - 2020-02-27
### Added
- Documentation for `Plot`
- A feature (`orca`) that enables `Plot` conversion to the following output formats: png, jpeg, webp, svg, pdf and eps
    - This **optional** feature is implemented in the `plotly_orca` crate
- Box plot support
- Heat map support
- Contour plot support
- More histogram examples
- Filled lines example

### Changed
- Using specific Plotly.js version: https://cdn.plot.ly/plotly-1.52.2.js
- `Plot::add_layout` changed to `Plot::set_layout` as there is only one layout per `Plot`
- `TraceSerialize` renamed to `Trace`
- `Plot::show_jpg` renamed to `Plot::show_jpeg` for consistency
- Removed HexColor type. Hex color input is still supported using `String`s or string slices
- Refactored project structure:
    - All plots `Trace`s are now accessible from the main namespace `plotly::`.
    - Enums and structs common to more than 1 plot type and/or the `Layout` now live in `plotly::common::`
    - Internal methods and structs that are not considered part of the public API are now in `plotly::private::`

### Fixed
- Color serialization was operating correctly only on Rgb, Rgba and Hex colors ignoring the named colors

## [0.3.0] - 2020-02-23
### Added
- Surface plot
- More examples for scatter and line plots
### Changed
- Completed implementation of the following:
    - Scatter plot
    - Box plot
    - Scatter and Box plot with error bars
    - Candlestick plot
    - OHLC plot
- Extended README.md with a few basic examples
- The API is now based on the builder pattern
- Extended color set
### Fixed
- `Plot::show()` now correctly opens the plot in the default browser as is the case for MacOSX and Linux

## [0.2.1] - 2020-01-26
### Added
- The following plot types are incomplete but fairly functional:
    - Scatter plot
    - Box plot
    - Scatter and Box plot with error bars
    - Candlestick plot
    - OHLC plot
- Basic support for plot Layout
- Default template data for Layout
- Examples (See [example_plots.rs](examples/example_plots.rs))
    - [Line and scatter plot](https://plot.ly/javascript/line-and-scatter/#line-and-scatter-plot)
    - [Data labels hover](https://plot.ly/javascript/line-and-scatter/#data-labels-hover)
    - [Data labels on the plot](https://plot.ly/javascript/line-and-scatter/#data-labels-on-the-plot)
    - Geometric Brownian motion plot (scatter + line plot)
    - [Basic bar chart](https://plot.ly/javascript/bar-charts/#basic-bar-chart)
    - [Grouped bar chart](https://plot.ly/javascript/bar-charts/#grouped-bar-chart)
    - [Stacked bar chart](https://plot.ly/javascript/bar-charts/#stacked-bar-chart)
    - [Basic symmetric error bars](https://plot.ly/javascript/error-bars/#basic-symmetric-error-bars)
    - [Bar chart with error bars](https://plot.ly/javascript/error-bars/#bar-chart-with-error-bars)
    - [Simple candlestick chart](https://plot.ly/javascript/candlestick-charts/#simple-candlestick-chart)
    - Geometric Brownian motion candlestick plot
    - [Simple OHLC chart](https://plot.ly/javascript/ohlc-charts/#simple-ohlc-chart)
    - Geometric Brownian motion OHLC plot

## [0.1.0] - 2020-01-26
### Added
- Placeholder repository.
- Proof of concept implementation of a scatter plot.
