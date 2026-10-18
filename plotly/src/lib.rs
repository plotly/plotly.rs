//! # Plotly.rs
//!
//! A plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).
#![recursion_limit = "256"] // lets us use a large serde_json::json! macro for
                            // testing crate::layout::Axis
extern crate askama;
extern crate rand;
extern crate serde;

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
pub mod export;
pub mod layout;
pub mod plot;
pub mod traces;

pub use common::color;
pub use configuration::Configuration;
pub use layout::Layout;
pub use plot::{Plot, Trace, Traces};
// Also provide easy access to modules which contain additional trace-specific types
pub use traces::{
    bar_polar, box_plot, choropleth, choropleth_map, cone, contour, density_map, funnel,
    funnel_area, heat_map, histogram, histogram2d, icicle, image, indicator, isosurface, mesh3d,
    parcats, sankey, scatter, scatter3d, scatter_map, scatter_mapbox, splom, stream_tube, sunburst,
    surface, treemap, violin, volume, waterfall,
};
// Bring the different trace types into the top-level scope
pub use traces::{
    Bar, BarPolar, BoxPlot, Candlestick, Choropleth, ChoroplethMap, Cone, Contour, DensityMap,
    DensityMapbox, Funnel, FunnelArea, HeatMap, Histogram, Histogram2d, Icicle, Image, Indicator,
    IsoSurface, Mesh3D, Ohlc, Parcats, Pie, Sankey, Scatter, Scatter3D, ScatterGeo, ScatterMap,
    ScatterMapbox, ScatterPolar, Splom, StreamTube, Sunburst, Surface, Table, Treemap, Violin,
    Volume, Waterfall,
};

pub trait Restyle: serde::Serialize {}
pub trait Relayout {}

#[cfg(feature = "plotly_static")]
pub use plotly_static::{self, ImageFormat};

// Public prelude for ergonomic imports in examples and user code
pub mod prelude {
    #[cfg(feature = "plotly_static")]
    pub use crate::export::r#async::ExporterAsyncExt;
    #[cfg(feature = "plotly_static")]
    pub use crate::export::sync::ExporterSyncExt;
    #[cfg(feature = "plotly_static")]
    pub use crate::plotly_static::ImageFormat;
}

// Not public API.
#[doc(hidden)]
mod private;
