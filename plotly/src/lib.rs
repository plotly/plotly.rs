//! # Plotly.rs
//!
//! A plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).
#![recursion_limit = "256"] // lets us use a large serde_json::json! macro for testing crate::layout::Axis
extern crate rand;
extern crate rinja;
extern crate serde;

#[cfg(all(feature = "kaleido", target_family = "wasm"))]
compile_error!(
    r#"The "kaleido" feature is not available on "wasm" targets. Please compile without this feature for the wasm target family."#
);

#[cfg(feature = "plotly_ndarray")]
pub mod ndarray;
#[cfg(feature = "plotly_ndarray")]
pub use crate::ndarray::ArrayTraces;

#[cfg(target_family = "wasm")]
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
pub use traces::{
    box_plot, contour, heat_map, histogram, image, mesh3d, sankey, scatter, scatter3d,
    scatter_mapbox, surface,
};
// Bring the different trace types into the top-level scope
pub use traces::{
    Bar, BoxPlot, Candlestick, Contour, DensityMapbox, HeatMap, Histogram, Image, Mesh3D, Ohlc,
    Pie, Sankey, Scatter, Scatter3D, ScatterMapbox, ScatterPolar, Surface, Table,
};

pub trait Restyle: serde::Serialize {}
pub trait Relayout {}

// Not public API.
#[doc(hidden)]
mod private;
