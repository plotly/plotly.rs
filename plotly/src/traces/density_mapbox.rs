//! Density mapbox scatter plot

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::common::{
    ColorBar, ColorScale, Dim, HoverInfo, Label, LegendGroupTitle, Line, PlotType, Visible,
};
use crate::private::{NumOrString, NumOrStringCollection};
use crate::Trace;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct DensityMapbox<Lat, Lon, Z>
where
    Lat: Serialize + Clone,
    Lon: Serialize + Clone,
    Z: Serialize + Clone,
{
    #[field_setter(default = "PlotType::DensityMapbox")]
    r#type: PlotType,
    /// Sets the trace name. The trace name appear as the legend item and on
    /// hover.
    name: Option<String>,
    /// Determines whether or not this trace is visible. If
    /// `Visible::LegendOnly`, the trace is not drawn, but can appear as a
    /// legend item (provided that the legend itself is visible).
    visible: Option<Visible>,

    /// Determines whether or not an item corresponding to this trace is shown
    /// in the legend.
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,

    /// Sets the legend rank for this trace. Items and groups with smaller ranks
    /// are presented on top/left side while with `"reversed"
    /// `legend.trace_order` they are on bottom/right side. The default
    /// legendrank is 1000, so that you can use ranks less than 1000 to
    /// place certain items before all unranked items, and ranks greater
    /// than 1000 to go after all unranked items.
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    /// Sets the legend group for this trace. Traces part of the same legend
    /// group show/hide at the same time when toggling legend items.
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    /// Set and style the title to appear for the legend group.
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,

    /// Line display properties.
    line: Option<Line>,

    /// Assigns id labels to each datum. These ids are for object constancy of
    /// data points during animation.
    ids: Option<Vec<String>>,

    lat: Option<Vec<Lat>>,
    lon: Option<Vec<Lon>>,
    z: Option<Vec<Z>>,

    /// Sets the opacity of the trace.
    opacity: Option<f64>,

    /// Sets text elements associated with each (lat,lon,z) triplet. If a single
    /// string, the same string appears over all the data points. If an array of
    /// strings, the items are mapped in order to the data points. To be seen,
    /// the trace `hover_info` must contain a "text" flag.
    text: Option<Dim<String>>,

    /// Sets hover text elements associated with each (lat,lon,z) triplet. If a
    /// single string, the same string appears over all the data points. If an
    /// array of strings, the items are mapped in order to the data points. To
    /// be seen, the trace `hover_info` must contain a "text" flag.
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,

    /// Determines which trace information appears on hover.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,

    /// Template string used for rendering the information that appear on hover
    /// box. Note that this will override `hover_info`. Variables are inserted
    /// using %{variable}, for example "y: %{y}".
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,

    /// Fallback string used for rendering the information that appear on hover
    /// box when the `hover_template` cannot be evaluated.
    #[serde(rename = "hovertemplatefallback")]
    hover_template_fallback: Option<Dim<String>>,

    /// Properties of label displayed on mouse hover.
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,

    /// Assigns extra meta information associated with this trace that can be
    /// used in various text attributes.
    meta: Option<NumOrString>,

    /// Assigns extra data each datum. This may be useful when listening to
    /// hover, click and selection events. Note that, "scatter" traces also
    /// appends customdata items in the markers DOM elements.
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,

    /// Sets a reference between this trace's data coordinates and a mapbox
    /// subplot. If "mapbox" (the default value), the data refer to
    /// `layout.mapbox`. If "mapbox2", the data refer to `layout.mapbox2`, and
    /// so on.
    subplot: Option<String>,

    /// Determines whether or not the color domain is computed
    /// with respect to the input data (here in `z`) or the bounds set
    /// in `zmin` and `zmax`. Defaults to false when `zmin` and `zmax` are
    /// set by the user.
    zauto: Option<bool>,

    /// Sets the upper bound of the color domain. Value should have the
    /// same units as in `z` and if set, `zmin` must be set as well.
    zmax: Option<Z>,

    zmid: Option<Z>,

    zmin: Option<Z>,

    zoom: Option<u8>,

    radius: Option<u8>,

    /// Sets the colorscale. The colorscale must be an array containing arrays
    /// mapping a normalized value to an rgb, rgba, hex, hsl, hsv, or named
    /// color string.
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,

    /// Determines whether the colorscale is a default palette
    /// (`auto_color_scale: true`) or the palette determined by `color_scale`.
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,

    /// Reverses the color mapping if true. If true, `zmin` will correspond to
    /// the last color in the array and `zmax` will correspond to the first
    /// color.
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,

    /// Determines whether or not a colorbar is displayed for this trace.
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,

    /// Sets the colorbar properties.
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,

    /// Sets a reference to a shared color axis (e.g. "coloraxis",
    /// "coloraxis2").
    #[serde(rename = "coloraxis")]
    color_axis: Option<String>,

    /// Determines if this trace's layer is displayed below the layer with the
    /// given id. By default, density traces are placed below the first layer of
    /// type symbol. Set `below` to "" to place it above every other layer.
    below: Option<String>,

    /// Controls persistence of some user-driven changes to the trace: `visible`
    /// only. Defaults to `layout.uirevision`.
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<Lat, Lon, Z> DensityMapbox<Lat, Lon, Z>
where
    Lat: Serialize + Clone + std::default::Default, // TODO why is "+ Default" necessary?
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

impl<Lat, Lon, Z> Trace for DensityMapbox<Lat, Lon, Z>
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
    fn serialize_density_mapbox() {
        let density_mapbox = DensityMapbox::new(vec![45.5017], vec![-73.5673], vec![1.0])
            .name("name")
            .visible(Visible::True)
            .show_legend(true)
            .legend_rank(1000)
            .legend_group("legend group")
            .zoom(5)
            .radius(20)
            .opacity(0.5)
            .hover_text_array(vec!["Montreal"])
            .hover_info(HoverInfo::Text)
            .hover_template("%{lat}, %{lon}")
            .custom_data(vec!["Montreal"])
            .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
            .show_scale(true)
            .reverse_scale(true)
            .below("");
        let expected = json!({
            "type": "densitymapbox",
            "lat": [45.5017],
            "lon": [-73.5673],
            "z": [1.0],
            "name": "name",
            "visible": true,
            "showlegend": true,
            "legendrank": 1000,
            "legendgroup": "legend group",
            "opacity": 0.5,
            "zoom": 5,
            "radius": 20,
            "hovertext": ["Montreal"],
            "hoverinfo": "text",
            "hovertemplate": "%{lat}, %{lon}",
            "customdata": ["Montreal"],
            "colorscale": "Viridis",
            "showscale": true,
            "reversescale": true,
            "below": "",
        });
        assert_eq!(to_value(density_mapbox.clone()).unwrap(), expected);
    }
}
