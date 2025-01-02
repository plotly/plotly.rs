//! Density mapbox scatter plot

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::common::{LegendGroupTitle, Line, PlotType, Visible};
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

    lat: Option<Vec<Lat>>,
    lon: Option<Vec<Lon>>,
    z: Option<Vec<Z>>,

    /// Sets the opacity of the trace.
    opacity: Option<f64>,

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
    //color_continuous_scale: Option<HashMap<Z, NamedColor>>,
    //color_continuous_midpoint: Option<ContinuousColorScale>,
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
            .opacity(0.5);
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
        });
        assert_eq!(to_value(density_mapbox.clone()).unwrap(), expected);
    }
}
