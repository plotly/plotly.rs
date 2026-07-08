//! Density heatmap trace for the MapLibre `map` subplot.

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::common::{
    ColorBar, ColorScale, Dim, HoverInfo, Label, LegendGroupTitle, PlotType, Visible,
};
use crate::private::{NumOrString, NumOrStringCollection};
use crate::Trace;

/// Construct a density heatmap trace drawn on the MapLibre `map` subplot
/// (configured via [`LayoutMap`](crate::layout::LayoutMap)).
///
/// `DensityMap` is the modern MapLibre-based counterpart to
/// [`DensityMapbox`](crate::DensityMapbox): it draws a weighted kernel-density
/// heatmap from `lat`/`lon` points (with an optional per-point weight `z`) onto
/// the `layout.map` subplot.
///
/// # Examples
///
/// ```
/// use plotly::DensityMap;
///
/// let trace = DensityMap::new(vec![45.5017], vec![-73.5673], vec![1.0])
///     .radius(20)
///     .name("montreal");
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct DensityMap<Lat, Lon, Z>
where
    Lat: Serialize + Clone,
    Lon: Serialize + Clone,
    Z: Serialize + Clone,
{
    #[field_setter(default = "PlotType::DensityMap")]
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

    /// Sets the latitude coordinates (in degrees North).
    lat: Option<Vec<Lat>>,
    /// Sets the longitude coordinates (in degrees East).
    lon: Option<Vec<Lon>>,
    /// Sets the points' weight. For example, a value of 10 would be equivalent
    /// to having 10 points of weight 1 in the same spot.
    z: Option<Vec<Z>>,
    /// Sets the radius of influence of one `lat`/`lon` point in pixels.
    /// Increasing the value makes the density map smoother, but less detailed.
    radius: Option<u8>,
    /// Sets the opacity of the trace.
    opacity: Option<f64>,

    /// Sets the text elements associated with each (lat,lon,z) triplet.
    text: Option<Dim<String>>,
    /// Sets the hover text elements associated with each (lat,lon,z) triplet.
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

    /// Assigns extra meta information associated with this trace.
    meta: Option<NumOrString>,
    /// Assigns extra data to each datum.
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,
    /// Controls persistence of some user-driven changes to the trace.
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<Lat, Lon, Z> DensityMap<Lat, Lon, Z>
where
    Lat: Serialize + Clone + std::default::Default,
    Lon: Serialize + Clone + std::default::Default,
    Z: Serialize + Clone + std::default::Default,
{
    pub fn new(lat: Vec<Lat>, lon: Vec<Lon>, z: Vec<Z>) -> Box<Self> {
        Box::new(Self {
            lat: Some(lat),
            lon: Some(lon),
            z: Some(z),
            ..Default::default()
        })
    }
}

impl<Lat, Lon, Z> Trace for DensityMap<Lat, Lon, Z>
where
    Lat: Serialize + Clone,
    Lon: Serialize + Clone,
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
    fn serialize_density_map() {
        let trace = DensityMap::new(vec![45.5017], vec![-73.5673], vec![1.0])
            .name("montreal")
            .radius(20)
            .opacity(0.5)
            .hover_text_array(vec!["Montreal"])
            .hover_info(HoverInfo::Text)
            .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
            .show_scale(true)
            .subplot("map")
            .below("");

        let expected = json!({
            "type": "densitymap",
            "lat": [45.5017],
            "lon": [-73.5673],
            "z": [1.0],
            "name": "montreal",
            "radius": 20,
            "opacity": 0.5,
            "hovertext": ["Montreal"],
            "hoverinfo": "text",
            "colorscale": "Viridis",
            "showscale": true,
            "subplot": "map",
            "below": "",
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
