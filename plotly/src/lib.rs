//! # Plotly.rs
//!
//! A plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).
#![recursion_limit = "256"] // lets us use a large serde_json::json! macro for testing crate::layout::Axis
extern crate askama;
extern crate rand;
extern crate serde;

#[cfg(all(feature = "kaleido", feature = "wasm"))]
compile_error!(
    r#"The "kaleido" and "wasm" features are mutually exclusive and cannot be activated at the same time. Please disable one or the other."#
);

#[cfg(feature = "plotly_ndarray")]
pub mod ndarray;
#[cfg(feature = "plotly_ndarray")]
pub use crate::ndarray::ArrayTraces;

#[cfg(feature = "wasm")]
pub mod bindings;

pub mod common;
pub mod configuration;
pub mod layout;
pub mod plot;
pub mod traces;

pub use common::color;
pub use configuration::Configuration;
pub use layout::Layout;
pub use plot::{ImageFormat, Plot, Trace};
// Also provide easy access to modules which contain additional trace-specific types
pub use traces::{box_plot, contour, histogram, image, mesh3d, sankey, scatter_mapbox, surface};
// Bring the different trace types into the top-level scope
pub use traces::{
    Bar, BoxPlot, Candlestick, Contour, DensityMapbox, HeatMap, Histogram, Image, Mesh3D, Ohlc,
    Sankey, Scatter, Scatter3D, ScatterMapbox, ScatterPolar, Surface,
};

pub trait Restyle: serde::Serialize {}
pub trait Relayout {}

// Not public API.
#[doc(hidden)]
mod private;
