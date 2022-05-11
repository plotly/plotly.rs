//! # Plotly.rs
//!
//! A plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).
#![recursion_limit = "256"] // lets us use a large serde_json::json! macro for testing crate::layout::Axis
#![allow(dead_code)]
extern crate askama;
extern crate rand;
extern crate serde;

pub mod ndarray;

#[cfg(feature = "wasm")]
pub mod bindings;

pub mod configuration;
pub mod layout;
pub mod plot;

pub mod themes;

pub mod bar;
pub mod box_plot;
pub mod candlestick;
pub mod common;
pub mod contour;
pub mod heat_map;
pub mod histogram;
pub mod ohlc;
pub mod sankey;
pub mod scatter;
pub mod scatter3d;
pub mod scatter_polar;
pub mod surface;

pub use crate::configuration::Configuration;
pub use crate::layout::Layout;
pub use crate::plot::ImageFormat;
pub use crate::plot::Plot;

pub use crate::bar::Bar;
pub use crate::box_plot::BoxPlot;
pub use crate::candlestick::Candlestick;
pub use crate::contour::Contour;
pub use crate::heat_map::HeatMap;
pub use crate::histogram::Histogram;
pub use crate::ohlc::Ohlc;
pub use crate::sankey::Sankey;
pub use crate::scatter::Scatter;
pub use crate::scatter3d::Scatter3D;
pub use crate::scatter_polar::ScatterPolar;
pub use crate::surface::Surface;

pub use crate::common::color::NamedColor;
pub use crate::common::color::Rgb;
pub use crate::common::color::Rgba;

pub use crate::plot::Trace;

#[cfg(feature = "plotly_ndarray")]
pub use crate::ndarray::ArrayTraces;

// Not public API.
#[doc(hidden)]
pub mod private;
