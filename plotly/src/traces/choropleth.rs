//! Choropleth trace for the `geo` subplot.

use plotly_derive::FieldSetter;
use serde::Serialize;
use serde_json::Value;

use crate::common::{
    ColorBar, ColorScale, Dim, HoverInfo, Label, LegendGroupTitle, Line, PlotType, Visible,
};
use crate::private::{NumOrString, NumOrStringCollection};
use crate::Trace;

/// Determines the set of locations used to match entries in `locations` to
/// regions on the map.
#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum LocationMode {
    #[serde(rename = "ISO-3")]
    Iso3,
    #[serde(rename = "USA-states")]
    UsaStates,
    #[serde(rename = "country names")]
    CountryNames,
    #[serde(rename = "geojson-id")]
    GeoJsonId,
}

/// Marker styling for choropleth traces. Unlike the scatter marker, the
/// choropleth marker only exposes the region boundary `line` and per-region
/// `opacity`.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Marker {
    /// Sets the line (region boundary) styling.
    line: Option<Line>,
    /// Sets the opacity of the regions.
    opacity: Option<Dim<f64>>,
}

impl Marker {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct SelectionMarker {
    opacity: Option<f64>,
}

/// Styles the regions of `selected`/`unselected` points.
#[derive(Serialize, Clone, Debug, Default)]
pub struct Selection {
    marker: SelectionMarker,
}

impl Selection {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the marker opacity of un/selected regions.
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.marker.opacity = Some(opacity);
        self
    }
}

/// Construct a choropleth trace, drawn on the `geo` subplot.
///
/// # Examples
///
/// ```
/// use plotly::Choropleth;
/// use plotly::choropleth::LocationMode;
///
/// let trace = Choropleth::new(vec!["CAN", "USA", "MEX"], vec![1.0, 2.0, 3.0])
///     .location_mode(LocationMode::Iso3)
///     .name("countries");
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Choropleth<Loc, Z>
where
    Loc: Serialize + Clone,
    Z: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Choropleth")]
    r#type: PlotType,
    /// Sets the trace name. The trace name appears as the legend item and on
    /// hover.
    name: Option<String>,
    /// Determines whether or not this trace is visible.
    visible: Option<Visible>,
    /// Determines whether or not an item corresponding to this trace is shown
    /// in the legend.
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    /// Sets the legend rank for this trace.
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    /// Sets the legend group for this trace.
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    /// Set and style the title to appear for the legend group.
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    /// Assigns id labels to each datum.
    ids: Option<Vec<String>>,

    /// Sets the coordinates via location IDs or names. See `location_mode` for
    /// more info.
    locations: Option<Vec<Loc>>,
    /// Sets the color values, one per location.
    z: Option<Vec<Z>>,
    /// Determines the set of locations used to match entries in `locations` to
    /// regions on the map.
    #[serde(rename = "locationmode")]
    location_mode: Option<LocationMode>,
    /// Sets optional GeoJSON data associated with this trace. Accepts either a
    /// URL string pointing to a GeoJSON file, or an inline GeoJSON object.
    /// Used with `location_mode` [`LocationMode::GeoJsonId`].
    geojson: Option<Value>,
    /// Sets the key in GeoJSON features which is used as id to match the items
    /// included in the `locations` array. Defaults to `id`.
    #[serde(rename = "featureidkey")]
    feature_id_key: Option<String>,

    /// Sets the text elements associated with each location.
    text: Option<Dim<String>>,
    /// Sets the hover text elements associated with each location.
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Determines which trace information appears on hover.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    /// Template string used for rendering the information that appears on the
    /// hover box.
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertemplatefallback")]
    hover_template_fallback: Option<Dim<String>>,
    /// Properties of the hover label.
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,

    /// Determines whether or not the color domain is computed with respect to
    /// the input data (here in `z`) or the bounds set in `zmin` and `zmax`.
    #[serde(rename = "zauto")]
    z_auto: Option<bool>,
    /// Sets the lower bound of the color domain.
    #[serde(rename = "zmin")]
    z_min: Option<f64>,
    /// Sets the upper bound of the color domain.
    #[serde(rename = "zmax")]
    z_max: Option<f64>,
    /// Sets the mid-point of the color domain by scaling `zmin` and/or `zmax`
    /// to be equidistant to this point.
    #[serde(rename = "zmid")]
    z_mid: Option<f64>,
    /// Sets the colorscale.
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    /// Determines whether the colorscale is a default palette
    /// (`autocolorscale: true`).
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    /// Reverses the color mapping if true.
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    /// Determines whether or not a colorbar is displayed for this trace.
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
    /// Properties of the colorbar.
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
    /// Sets a reference to a shared color axis (e.g. `"coloraxis"`,
    /// `"coloraxis2"`).
    #[serde(rename = "coloraxis")]
    color_axis: Option<String>,

    /// Sets a reference to the `geo` subplot this trace is drawn on. Defaults
    /// to `"geo"`.
    geo: Option<String>,
    /// Sets the marker (region boundary line and opacity) styling.
    marker: Option<Marker>,
    /// Styles the regions of selected points.
    selected: Option<Selection>,
    /// Styles the regions of unselected points.
    unselected: Option<Selection>,
    /// Array of integer indices into the data points that are selected.
    #[serde(rename = "selectedpoints")]
    selected_points: Option<Vec<usize>>,

    /// Assigns extra meta information associated with this trace.
    meta: Option<NumOrString>,
    /// Assigns extra data to each datum.
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,
    /// Controls persistence of some user-driven changes to the trace.
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<Loc, Z> Choropleth<Loc, Z>
where
    Loc: Serialize + Clone + std::default::Default,
    Z: Serialize + Clone + std::default::Default,
{
    pub fn new(locations: Vec<Loc>, z: Vec<Z>) -> Box<Self> {
        Box::new(Self {
            locations: Some(locations),
            z: Some(z),
            ..Default::default()
        })
    }
}

impl<Loc, Z> Trace for Choropleth<Loc, Z>
where
    Loc: Serialize + Clone,
    Z: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::ColorScalePalette;

    #[test]
    #[rustfmt::skip]
    fn serialize_location_mode() {
        assert_eq!(to_value(LocationMode::Iso3).unwrap(), json!("ISO-3"));
        assert_eq!(to_value(LocationMode::UsaStates).unwrap(), json!("USA-states"));
        assert_eq!(to_value(LocationMode::CountryNames).unwrap(), json!("country names"));
        assert_eq!(to_value(LocationMode::GeoJsonId).unwrap(), json!("geojson-id"));
    }

    #[test]
    fn serialize_choropleth() {
        let trace = Choropleth::new(vec!["CAN", "USA", "MEX"], vec![1.0, 2.0, 3.0])
            .name("countries")
            .visible(Visible::True)
            .show_legend(false)
            .location_mode(LocationMode::Iso3)
            .feature_id_key("id")
            .z_min(0.0)
            .z_max(5.0)
            .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
            .reverse_scale(false)
            .show_scale(true)
            .color_bar(ColorBar::new())
            .marker(Marker::new().line(Line::new().width(0.5)))
            .selected(Selection::new().opacity(1.0))
            .geo("geo");

        let expected = json!({
            "type": "choropleth",
            "locations": ["CAN", "USA", "MEX"],
            "z": [1.0, 2.0, 3.0],
            "name": "countries",
            "visible": true,
            "showlegend": false,
            "locationmode": "ISO-3",
            "featureidkey": "id",
            "zmin": 0.0,
            "zmax": 5.0,
            "colorscale": "Viridis",
            "reversescale": false,
            "showscale": true,
            "colorbar": {},
            "marker": {"line": {"width": 0.5}},
            "selected": {"marker": {"opacity": 1.0}},
            "geo": "geo",
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn serialize_choropleth_inline_geojson() {
        let trace = Choropleth::new(vec!["a"], vec![1.0])
            .location_mode(LocationMode::GeoJsonId)
            .geojson(json!({"type": "FeatureCollection", "features": []}));

        let value = to_value(trace).unwrap();
        assert_eq!(
            value["geojson"],
            json!({"type": "FeatureCollection", "features": []})
        );
        assert_eq!(value["locationmode"], json!("geojson-id"));
    }
}
