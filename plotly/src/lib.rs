//! # Plotly.rs
//!
//! A plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).
#![recursion_limit = "256"] // lets us use a large serde_json::json! macro for testing crate::layout::Axis
extern crate askama;
extern crate rand;
extern crate serde;

#[cfg(feature = "plotly_ndarray")]
pub mod ndarray;

#[cfg(feature = "wasm")]
pub mod bindings;

pub mod common;
pub mod configuration;
pub mod layout;
pub mod plot;
pub mod traces;

pub use common::color;
use common::Dim;
pub use configuration::Configuration;
pub use layout::Layout;
pub use plot::{ImageFormat, Plot, Trace};

// Bring the different trace types into the top-level scope
pub use traces::{
    Bar, BoxPlot, Candlestick, Contour, HeatMap, Histogram, Ohlc, Sankey, Scatter, Scatter3D,
    ScatterPolar, Surface,
};
// Also provide easy access to modules which contain additional trace-specific types
pub use traces::{box_plot, contour, histogram, sankey, surface};

#[cfg(feature = "plotly_ndarray")]
pub use crate::ndarray::ArrayTraces;

pub trait Restyle: serde::Serialize {}
pub trait Relayout {}

pub trait GetInner {
    type Inner;
}

impl<T> GetInner for Option<T> {
    type Inner = T;
}

impl<T: serde::Serialize> GetInner for Dim<T> {
    type Inner = T;
}

// Not public API.
#[doc(hidden)]
mod private;
