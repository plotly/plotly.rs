//! # Plotly.rs
//!
//! A plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).
//!
//! ## Feature Deprecation Notice
//!
//! The `kaleido` and `kaleido_download` features are deprecated since version
//! 0.13.0 and will be removed in version 0.14.0. Please migrate to the
//! `plotly_static` and `plotly_static_download` features instead.
#![recursion_limit = "256"] // lets us use a large serde_json::json! macro for testing crate::layout::Axis
extern crate askama;
extern crate rand;
extern crate serde;

#[cfg(feature = "kaleido")]
#[deprecated(
    since = "0.13.0",
    note = "kaleido feature is deprecated and will be removed in version 0.14.0. Use plotly_static feature instead"
)]
const _KALEIDO_DEPRECATED: () = ();

#[cfg(feature = "kaleido_download")]
#[deprecated(
    since = "0.13.0",
    note = "kaleido_download feature is deprecated and will be removed in version 0.14.0. Use plotly_static_download feature instead"
)]
const _KALEIDO_DOWNLOAD_DEPRECATED: () = ();

#[cfg(all(feature = "kaleido", target_family = "wasm"))]
compile_error!(
    r#"The "kaleido" feature is not available on "wasm" targets. Please compile without this feature for the wasm target family."#
);

#[cfg(all(feature = "kaleido", feature = "plotly_static"))]
compile_error!(
    r#"The "kaleido" feature and "plotly_static" are conflictings. Please select only one of them."#
);

#[cfg(feature = "plotly_ndarray")]
pub mod ndarray;
#[cfg(feature = "plotly_ndarray")]
pub use crate::ndarray::ArrayTraces;

#[cfg(target_family = "wasm")]
pub mod bindings;

#[cfg(target_family = "wasm")]
pub mod callbacks;

pub mod common;
pub mod configuration;
pub mod layout;
pub mod plot;
pub mod traces;

pub use common::color;
pub use configuration::Configuration;
pub use layout::Layout;
pub use plot::{Plot, Trace, Traces};
#[cfg(feature = "kaleido")]
pub use plotly_kaleido::ImageFormat;
#[cfg(feature = "plotly_static")]
pub use plotly_static;
// Also provide easy access to modules which contain additional trace-specific types
pub use traces::{
    box_plot, contour, heat_map, histogram, image, mesh3d, sankey, scatter, scatter3d,
    scatter_mapbox, surface,
};
// Bring the different trace types into the top-level scope
pub use traces::{
    Bar, BoxPlot, Candlestick, Contour, DensityMapbox, HeatMap, Histogram, Image, Mesh3D, Ohlc,
    Pie, Sankey, Scatter, Scatter3D, ScatterGeo, ScatterMapbox, ScatterPolar, Surface, Table,
};

pub trait Restyle: serde::Serialize {}
pub trait Relayout {}

// Not public API.
#[doc(hidden)]
mod private;
