//! Choropleth trace for the MapLibre `map` subplot.

use plotly_derive::FieldSetter;
use serde::Serialize;
use serde_json::Value;

use crate::common::{
    ColorBar, ColorScale, Dim, HoverInfo, Label, LegendGroupTitle, PlotType, Visible,
};
use crate::private::{NumOrString, NumOrStringCollection};
use crate::traces::choropleth::{Marker, Selection};
use crate::Trace;

/// Construct a choropleth trace drawn on the MapLibre `map` subplot
/// (configured via [`LayoutMap`](crate::layout::LayoutMap)).
///
/// Unlike [`Choropleth`](crate::Choropleth), the `map` variant is GeoJSON-only:
/// `locations` are matched against features in `geojson` via `feature_id_key`,
/// so there is no `location_mode`.
///
/// # Examples
///
/// ```
/// use plotly::ChoroplethMap;
/// use serde_json::json;
///
/// let trace = ChoroplethMap::new(vec!["AL", "AK"], vec![1.0, 2.0])
///     .geojson(json!({"type": "FeatureCollection", "features": []}))
///     .feature_id_key("properties.code")
///     .name("states");
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct ChoroplethMap<Loc, Z>
where
    Loc: Serialize + Clone,
    Z: Serialize + Clone,
{
    #[field_setter(default = "PlotType::ChoroplethMap")]
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

    /// Sets the coordinates via location IDs. Matches the items in `locations`
    /// to features in `geojson` using `feature_id_key`.
    locations: Option<Vec<Loc>>,
    /// Sets the color values, one per location.
    z: Option<Vec<Z>>,
    /// Sets the GeoJSON data associated with this trace. Accepts either a URL
    /// string pointing to a GeoJSON file, or an inline GeoJSON object.
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
    /// Sets the mid-point of the color domain.
    #[serde(rename = "zmid")]
    z_mid: Option<f64>,
    /// Sets the colorscale.
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    /// Determines whether the colorscale is a default palette.
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
    /// Sets a reference to a shared color axis.
    #[serde(rename = "coloraxis")]
    color_axis: Option<String>,

    /// Sets a reference to the `map` subplot this trace is drawn on. Defaults
    /// to `"map"`.
    subplot: Option<String>,
    /// Determines if this trace's layer is inserted below the layer with the
    /// specified ID. By default, the layer is inserted above every existing
    /// layer.
    below: Option<String>,
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

impl<Loc, Z> ChoroplethMap<Loc, Z>
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

impl<Loc, Z> Trace for ChoroplethMap<Loc, Z>
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
    use crate::common::Line;

    #[test]
    fn serialize_choropleth_map() {
        let trace = ChoroplethMap::new(vec!["AL", "AK"], vec![1.0, 2.0])
            .name("states")
            .geojson(json!({"type": "FeatureCollection", "features": []}))
            .feature_id_key("properties.code")
            .color_scale(ColorScale::Palette(
                crate::common::ColorScalePalette::Bluered,
            ))
            .show_scale(true)
            .marker(Marker::new().line(Line::new().width(1.0)))
            .subplot("map")
            .below("");

        let expected = json!({
            "type": "choroplethmap",
            "locations": ["AL", "AK"],
            "z": [1.0, 2.0],
            "name": "states",
            "geojson": {"type": "FeatureCollection", "features": []},
            "featureidkey": "properties.code",
            "colorscale": "Bluered",
            "showscale": true,
            "marker": {"line": {"width": 1.0}},
            "subplot": "map",
            "below": "",
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
