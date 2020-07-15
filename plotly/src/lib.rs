//! # Plotly.rs
//!
//! A plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).

#![allow(dead_code)]
extern crate askama;
extern crate rand;
extern crate serde;

pub mod layout;
pub mod plot;

pub mod bar;
pub mod box_plot;
pub mod candlestick;
pub mod common;
pub mod contour;
pub mod heat_map;
pub mod histogram;
pub mod ohlc;
pub mod scatter;
pub mod surface;

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
pub use crate::scatter::Scatter;
pub use crate::surface::Surface;

pub use crate::common::color::NamedColor;
pub use crate::common::color::Rgb;
pub use crate::common::color::Rgba;

pub use crate::plot::Trace;

// Not public API.
#[doc(hidden)]
pub mod private;
