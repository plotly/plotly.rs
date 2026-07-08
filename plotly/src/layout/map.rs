use plotly_derive::FieldSetter;
use serde::Serialize;

use super::mapbox::Center;
use crate::common::Domain;
use crate::private::NumOrString;

/// Sets the style of the MapLibre `map` subplot.
///
/// Note that the `map` subplot uses MapLibre GL and, unlike the legacy
/// `mapbox` subplot, does not require an access token for the bundled styles.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum MapStyle {
    Basic,
    #[serde(rename = "carto-darkmatter")]
    CartoDarkMatter,
    #[serde(rename = "carto-darkmatter-nolabels")]
    CartoDarkMatterNoLabels,
    CartoPositron,
    #[serde(rename = "carto-positron-nolabels")]
    CartoPositronNoLabels,
    CartoVoyager,
    #[serde(rename = "carto-voyager-nolabels")]
    CartoVoyagerNoLabels,
    Dark,
    Light,
    OpenStreetMap,
    Outdoors,
    Satellite,
    SatelliteStreets,
    Streets,
    WhiteBg,
}

/// Sets the bounds beyond which the `map` subplot cannot be panned.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct MapBounds {
    west: Option<f64>,
    east: Option<f64>,
    south: Option<f64>,
    north: Option<f64>,
}

impl MapBounds {
    pub fn new() -> Self {
        Default::default()
    }
}

/// The MapLibre-based `map` subplot, used by traces such as
/// [`ChoroplethMap`](crate::ChoroplethMap) and `scattermap`.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct LayoutMap {
    /// Sets the bearing angle of the map in degrees counter-clockwise from
    /// North.
    bearing: Option<f64>,
    /// Sets the bounds within which the map can be panned.
    bounds: Option<MapBounds>,
    /// Sets the latitude and longitude of the center of the map.
    center: Option<Center>,
    /// Sets the domain within which the map will be drawn.
    domain: Option<Domain>,
    /// Sets the pitch angle of the map in degrees, where `0` means
    /// perpendicular to the surface of the map.
    pitch: Option<f64>,
    /// Sets the style of the map.
    style: Option<MapStyle>,
    /// Sets the zoom level of the map.
    zoom: Option<f64>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl LayoutMap {
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
    fn serialize_map_style() {
        assert_eq!(to_value(MapStyle::Basic).unwrap(), json!("basic"));
        assert_eq!(to_value(MapStyle::CartoDarkMatter).unwrap(), json!("carto-darkmatter"));
        assert_eq!(to_value(MapStyle::CartoDarkMatterNoLabels).unwrap(), json!("carto-darkmatter-nolabels"));
        assert_eq!(to_value(MapStyle::CartoPositron).unwrap(), json!("carto-positron"));
        assert_eq!(to_value(MapStyle::CartoPositronNoLabels).unwrap(), json!("carto-positron-nolabels"));
        assert_eq!(to_value(MapStyle::CartoVoyager).unwrap(), json!("carto-voyager"));
        assert_eq!(to_value(MapStyle::CartoVoyagerNoLabels).unwrap(), json!("carto-voyager-nolabels"));
        assert_eq!(to_value(MapStyle::OpenStreetMap).unwrap(), json!("open-street-map"));
        assert_eq!(to_value(MapStyle::WhiteBg).unwrap(), json!("white-bg"));
        assert_eq!(to_value(MapStyle::Satellite).unwrap(), json!("satellite"));
        assert_eq!(to_value(MapStyle::SatelliteStreets).unwrap(), json!("satellite-streets"));
    }

    #[test]
    fn serialize_layout_map() {
        let map = LayoutMap::new()
            .bearing(30.0)
            .bounds(
                MapBounds::new()
                    .west(-10.0)
                    .east(10.0)
                    .south(-5.0)
                    .north(5.0),
            )
            .center(Center::new(45.0, -73.0))
            .pitch(15.0)
            .style(MapStyle::CartoPositron)
            .zoom(4.0);

        let expected = json!({
            "bearing": 30.0,
            "bounds": {"west": -10.0, "east": 10.0, "south": -5.0, "north": 5.0},
            "center": {"lat": 45.0, "lon": -73.0},
            "pitch": 15.0,
            "style": "carto-positron",
            "zoom": 4.0,
        });

        assert_eq!(to_value(map).unwrap(), expected);
    }
}
