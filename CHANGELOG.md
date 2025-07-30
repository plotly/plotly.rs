# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.13.5] - 2025-07-30

### Fixed  

- [[#345](https://github.com/plotly/plotly.rs/pull/345)] Re-export `ImageFormat` from `plotly_static` into `plotly` 

## [0.13.4] - 2025-07-17

### Fixed  

- [[#341](https://github.com/plotly/plotly.rs/pull/341)] Fix documentation related to `wasm` support 

### Changed
- [[#339](https://github.com/plotly/plotly.rs/pull/339)] Replace default Windows app with `explorer`

## [0.13.3] - 2025-07-12

### Changed
- [[#335](https://github.com/plotly/plotly.rs/pull/335)] Add minimal animation support 

## [0.13.2] - 2025-07-12

### Fixed  
- [[#336](https://github.com/plotly/plotly.rs/pull/336)] Fix `plotly_static` docs.rs build
- [[#327](https://github.com/plotly/plotly.rs/pull/327)] Fix book broken link

## [0.13.1] - 2025-07-07

### Fixed
- [[#326](https://github.com/plotly/plotly.rs/pull/326)] Fix book badges


## [0.13.0] - 2025-07-07

### Changed
- [[#277](https://github.com/plotly/plotly.rs/pull/277)] Removed `wasm` feature flag and put everything behind target specific dependencies. Added `.cargo/config.toml` for configuration flags needed by `getrandom` version 0.3 on `wasm` targets.
- [[#281](https://github.com/plotly/plotly.rs/pull/281)] Update to askama 0.13.0
- [[#287](https://github.com/plotly/plotly.rs/pull/287)] Added functionality for callbacks (using wasm)
- [[#289](https://github.com/plotly/plotly.rs/pull/289)] Fixes Kaleido static export for MacOS targets by removing `--disable-gpu` flag for MacOS 
- [[#291](https://github.com/plotly/plotly.rs/pull/291)] Remove `--disable-gpu` flag for Kaleido static-image generation for all targets. 
- [[#299](https://github.com/plotly/plotly.rs/pull/299)] Added customdata field to HeatMap
- [[#303](https://github.com/plotly/plotly.rs/pull/303)] Split layout mod.rs into modules
- [[#304](https://github.com/plotly/plotly.rs/pull/304)] Refactored examples to allow for generation of full html files 
- [[#320](https://github.com/plotly/plotly.rs/pull/320)] Make offline_js_sources function `pub`
- [[#321](https://github.com/plotly/plotly.rs/pull/321)] Make 'online_cdn_js' function also `pub` for consistenccy

### Added
- [[#311](https://github.com/plotly/plotly.rs/pull/311)] Add extra themes
- [[#312](https://github.com/plotly/plotly.rs/pull/312)] Add rangebreak to axis
- [[#313](https://github.com/plotly/plotly.rs/pull/313)] Add example of subplots with multiple traces
- [[#314](https://github.com/plotly/plotly.rs/pull/314)] Add axis range bounds support
- [[#317](https://github.com/plotly/plotly.rs/pull/317)] Show rangebreak usage with ints
- [[#318](https://github.com/plotly/plotly.rs/pull/318)] Add slider support
- [[#319](https://github.com/plotly/plotly.rs/pull/319)] Introduce `plotly_static` package for static export of plots using WebDriver driven browsers 


### Fixed
- [[#284](https://github.com/plotly/plotly.rs/pull/284)]  Allow plotly package to be compiled for android 
- [[#298](https://github.com/plotly/plotly.rs/pull/298)]  Added support for layout axis scaleratio
- [[#301](https://github.com/plotly/plotly.rs/pull/301)]  Added ScatterGeo trace and LayoutGeo support 

## [0.12.1] - 2025-01-02
### Fixed
- [[#269](https://github.com/plotly/plotly.rs/pull/269)] Fix publishing to crates.io issue

## [0.12.0] - 2025-01-02
### Changed
- [[#256](https://github.com/plotly/plotly.rs/pull/256)] Bump Cargo.toml edition to 2021
- [[#261](https://github.com/plotly/plotly.rs/pull/261)] Updated code of conduct

### Fixed
- [[#265](https://github.com/plotly/plotly.rs/pull/265)] Add Pie Chart trace
- [[#264](https://github.com/plotly/plotly.rs/issues/264)] Derive Deserialize on NamedColor, Rgb and Rgba
- [[#216](https://github.com/plotly/plotly.rs/issues/216)] Opt out of downloading Kaleido binaries and allow users to set Kaleido path via environment variable
- [[#259](https://github.com/plotly/plotly.rs/issues/259)] Mesh3d::new() has wrong signature
- [[#175](https://github.com/plotly/plotly.rs/issues/175)] Put multiple subplots in the same html - added an example using `build_html` crate.
- [[#228](https://github.com/plotly/plotly.rs/issues/228)] Redraw function seems to be broken - added example on generating responsive plots.

## [0.11.0] - 2024-12-06
### Changed
- [[#251](https://github.com/plotly/plotly.rs/pull/251)] Expose image data as String with `to_base64` and `to_svg` using Kaleido
- [[#245](https://github.com/plotly/plotly.rs/pull/245)] Change Contours size to be `f64` instead of `usize`
- [[#243](https://github.com/plotly/plotly.rs/pull/243)] Made `plotly_embed_js` embed all JS scripts when enabled.
   Renamed `use_cdn_plotly` to `use_cdn_js`.

### Fixed
- [[#248](https://github.com/plotly/plotly.rs/issues/248)] Book recipes do not render graphs
- [[#247](https://github.com/plotly/plotly.rs/issues/247)] Add function to export image (with Kaleido) as a b64 string
- [[#246](https://github.com/plotly/plotly.rs/pull/246)] Expose pattern fill api for histograms and bar charts
- [[#244](https://github.com/plotly/plotly.rs/pull/244)] Fix swapped x and y in the examples.
- [[#242](https://github.com/plotly/plotly.rs/issues/242)] Disable request for tex-svg.js file
- [[#237](https://github.com/plotly/plotly.rs/issues/237)] Add Categorical Axis ordering.

## [0.10.0] - 2024-09-16
### Added
- [[#231](https://github.com/plotly/plotly.rs/pull/231)] Added new `plotly_embed_js` feature to reduce binary sizes by not embedding `plotly.min.js` in the library unless explicitly enabled via the feature flag. Deprecates `use_local_plotly` in favor of explicit opt-in via the feature flag and introduce method `use_cdn_plotly` to allow users to use CDN version even behind the `plotly_embed_js` feature flag.

### Fixed
- [[#230](https://github.com/plotly/plotly.rs/pull/230)] Make Bar chart `width` and `offset` use `f64` values.

## [0.9.1] - 2024-09-06
### Added
- [[#217](https://github.com/plotly/plotly.rs/pull/217)] Added show_html(filename) method to bypass situations where accessing default `/tmp` is not possible, e.g., with in SNAP Firefox
- [[#227](https://github.com/plotly/plotly.rs/pull/227)] Switch from HTML template render from `askama` to `rinja`

### Fixed
- [[#232](https://github.com/plotly/plotly.rs/pull/232)] Generalize OS detection in the context of opening HTML pages with default app via `xdg-open`
- [[#233](https://github.com/plotly/plotly.rs/pull/233)] Fix mdBook code examples

## [0.9.0] - 2024-06-29
### Added
- [[#153](https://github.com/plotly/plotly.rs/pull/153)] Added `LayoutScene`.
- [[#154](https://github.com/plotly/plotly.rs/pull/154)] Improve ergonomics of `Title` and `LegendGroupTitle` structs: `new` method now takes no arguments as per other structs, whilst a new `with_text()` constructor is added for convenience. Where other structs contain a `Title` (and `LegendGroupTitle`), users  can now call the `title()` (and `legend_group_title()`) method with anything that `impl`s `Into<Title>`, viz. `String`, `&String`, `&str` and `Title`.
- [[#157](https://github.com/plotly/plotly.rs/pull/157)] Fix `HeatMap`'s setters for correctly setting `zmin`, `zmax` and `zmin` independent of `Z` input type.
- [[#159](https://github.com/plotly/plotly.rs/pull/159)] Make `heat_map` module public to expose `Smoothing enum`.
- [[#161](https://github.com/plotly/plotly.rs/pull/161)] Added `Axis` `scaleanchor` setter.
- [[#163](https://github.com/plotly/plotly.rs/pull/163)] Added `DensityMapbox`.
- [[#166](https://github.com/plotly/plotly.rs/pull/166)] Added subplot example with multiple titles.
- [[#178](https://github.com/plotly/plotly.rs/pull/178)] Fix setter for `Axis::matches` to take string arg.
- [[#180](https://github.com/plotly/plotly.rs/pull/180)] Add setter for `Mapbox::domain`.
- [[#181](https://github.com/plotly/plotly,rs/pull/181)] Fix compilation error when mixing the crate with `askama/with-axum` by adding `with-axum` feature.
- [[#207](https://github.com/plotly/plotly,rs/pull/207)] Add `Table` trace.
- [[#212](https://github.com/plotly/plotly.rs/pull/212)] Update LICENSE

## [0.8.4] - 2023-07-09
### Added
- [[#143](https://github.com/plotly/plotly.rs/pull/143)] Widen version range of `askama`.

### Fixed
- [[#129](https://github.com/plotly/plotly.rs/pull/129)] Fix issue for plots not showing in browser in Windows. Thanks to [@juanespj](https://github.com/juanespj) and [@M-NK-Y](https://github.com/M-NK-Y) for the PRs.
- [[#147](https://github.com/plotly/plotly.rs/pull/147)] Update documentation for `jupyter notebook` example.

## [0.8.3] - 2022-11-04
### Fixed
- [[#122](https://github.com/plotly/plotly.rs/pull/122)] Compilation error for the `wasm` feature.
- [[#123](https://github.com/plotly/plotly.rs/pull/123)] Compilation error for the `plotly_kaleido` feature.

## [0.8.2] - 2022-11-03
### Added
- [[#110](https://github.com/plotly/plotly.rs/pull/110)] `LegendGroupTitle` to existing traces.
- [[#88](https://github.com/plotly/plotly.rs/pull/88)] `Mesh3D`, `Image`, `ScatterMapbox` traces.

### Changed
- [[#113](https://github.com/plotly/plotly.rs/pull/113)] Refactored the structure of the examples to make them more accessible, whilst adding more examples e.g. for `wasm`.
- [[#115](https://github.com/plotly/plotly.rs/pull/115)] Simplify the function signature of Plot.to_inline_html() so that it just takes `Option<&str>` as an argument.

## [0.8.1] - 2022-09-25
### Added
- Button support (i.e. [updatemenus](https://plotly.com/javascript/reference/layout/updatemenus/)) contributed by [@sreenathkrishnan](https://github.com/sreenathkrishnan). Details and examples in this well written PR [#99](https://github.com/plotly/plotly.rs/pull/99).
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
- Improve implementation of `private::NumOrString` to support more primitive types ([Issue #47](https://github.com/plotly/plotly.rs/issues/47))
- Remove `private::TruthyEnum` in favour of a more robust way of serializing to `String` or `bool`
- Refactor `Color` module
- Refactored HTML templates with cleaner Javascript code
- `Plot::save()` is renamed to `Plot::write_image()`
- `Plot::show_{png | jpeg}` is now made generic via the new `Plot::show_image()` across static image formats with the `ImageFormat` enum
- `Plot::write_html()` now takes a filepath and saves the plot to that location
- `Plot::to_html()` now has similar behaviour to `Plot::to_inline_html()` and just returns a `String`
### Fixed
- Typos in `CONTRIBUTING.md`
- Serialization of `plotly_kaleido::PlotData` ([Issue #50](https://github.com/plotly/plotly.rs/issues/50))
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
- Shapes support ([documentation](https://plotly.github.io/plotly.rs/content/fundamentals/shapes.html)).
- Annotations support.
- Docstrings to `Scatter`.
- `ndarray` support ([documentation](https://plotly.github.io/plotly.rs/content/fundamentals/ndarray_support.html)).
- Jupyter lab and notebook support ([documentation](https://plotly.github.io/plotly.rs/content/fundamentals/jupyter_support.html)).
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
- [Plotly.rs Book](https://plotly.github.io/plotly.rs/).
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
