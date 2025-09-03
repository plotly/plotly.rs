use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::common::Domain;

/// Defines the latitude and longitude at which a map will be centered.
#[derive(Serialize, Clone, Debug)]
pub struct Center {
    lat: f64,
    lon: f64,
}

impl Center {
    /// Create a new instance of `Center`.
    ///
    /// `lat` is the number of degrees north, `lon` is the number of degrees
    /// east.
    pub fn new(lat: f64, lon: f64) -> Self {
        Center { lat, lon }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum MapboxStyle {
    #[serde(rename = "carto-darkmatter")]
    CartoDarkMatter,
    CartoPositron,
    OpenStreetMap,
    StamenTerrain,
    StamenToner,
    StamenWatercolor,
    WhiteBg,
    Basic,
    Streets,
    Outdoors,
    Light,
    Dark,
    Satellite,
    SatelliteStreets,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Mapbox {
    /// Sets the mapbox access token to be used for this mapbox map. Note that
    /// `access_token`s are only required when `style` (e.g with values: basic,
    /// streets, outdoors, light, dark, satellite, satellite-streets)
    /// and/or a layout layer references the Mapbox server.
    #[serde(rename = "accesstoken")]
    access_token: Option<String>,
    /// Sets the bearing angle of the map in degrees counter-clockwise from
    /// North.
    bearing: Option<f64>,
    /// Sets the latitude and longitude of the center of the map.
    center: Option<Center>,
    /// Sets the domain within which the mapbox will be drawn.
    domain: Option<Domain>,
    /// Sets the pitch angle of the map in degrees, where `0` means
    /// perpendicular to the surface of the map.
    pitch: Option<f64>,
    /// Sets the style of the map.
    style: Option<MapboxStyle>,
    /// Sets the zoom level of the map.
    zoom: Option<u8>,
}

impl Mapbox {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    #[rustfmt::skip]
    fn serialize_mapbox_style() {
        assert_eq!(to_value(MapboxStyle::CartoDarkMatter).unwrap(), json!("carto-darkmatter"));
        assert_eq!(to_value(MapboxStyle::CartoPositron).unwrap(), json!("carto-positron"));
        assert_eq!(to_value(MapboxStyle::OpenStreetMap).unwrap(), json!("open-street-map"));
        assert_eq!(to_value(MapboxStyle::StamenTerrain).unwrap(), json!("stamen-terrain"));
        assert_eq!(to_value(MapboxStyle::StamenToner).unwrap(), json!("stamen-toner"));
        assert_eq!(to_value(MapboxStyle::StamenWatercolor).unwrap(), json!("stamen-watercolor"));
        assert_eq!(to_value(MapboxStyle::WhiteBg).unwrap(), json!("white-bg"));
        assert_eq!(to_value(MapboxStyle::Basic).unwrap(), json!("basic"));
        assert_eq!(to_value(MapboxStyle::Streets).unwrap(), json!("streets"));
        assert_eq!(to_value(MapboxStyle::Outdoors).unwrap(), json!("outdoors"));
        assert_eq!(to_value(MapboxStyle::Light).unwrap(), json!("light"));
        assert_eq!(to_value(MapboxStyle::Dark).unwrap(), json!("dark"));
        assert_eq!(to_value(MapboxStyle::Satellite).unwrap(), json!("satellite"));
        assert_eq!(to_value(MapboxStyle::SatelliteStreets).unwrap(), json!("satellite-streets"));
    }
}
