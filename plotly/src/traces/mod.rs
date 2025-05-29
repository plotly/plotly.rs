//! The various supported traces

pub mod bar;
pub mod box_plot;
mod candlestick;
pub mod contour;
mod density_mapbox;
pub mod heat_map;
pub mod histogram;
pub mod image;
pub mod mesh3d;
mod ohlc;
pub mod pie;
pub mod sankey;
pub mod scatter;
pub mod scatter3d;
pub mod scatter_geo;
pub mod scatter_mapbox;
mod scatter_polar;
pub mod surface;
pub mod table;

pub use bar::Bar;
pub use box_plot::BoxPlot;
pub use candlestick::Candlestick;
pub use contour::Contour;
pub use density_mapbox::DensityMapbox;
pub use heat_map::HeatMap;
pub use histogram::Histogram;
pub use mesh3d::Mesh3D;
pub use ohlc::Ohlc;
pub use pie::Pie;
pub use sankey::Sankey;
pub use scatter::Scatter;
pub use scatter3d::Scatter3D;
pub use scatter_geo::ScatterGeo;
pub use scatter_mapbox::ScatterMapbox;
pub use scatter_polar::ScatterPolar;
pub use surface::Surface;
pub use table::Table;

pub use self::image::Image;
