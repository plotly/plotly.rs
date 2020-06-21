# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2020-XX-XX
### Added
- [Plotly for Rust Book](https://igiagkiozis.github.io/plotly/)
- TODO Add support for linking the plotly.js library to the remote source.
### Changed
- `Layout` complete.
### Fixed
- f

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