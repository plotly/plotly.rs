//! Heat map trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    common::{
        Calendar, ColorBar, ColorScale, Dim, HoverInfo, Label, LegendGroupTitle, PlotType, Visible,
    },
    Trace,
};

#[derive(Debug, Clone)]
pub enum Smoothing {
    Fast,
    Best,
    False,
}

impl Serialize for Smoothing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Fast => serializer.serialize_str("fast"),
            Self::Best => serializer.serialize_str("best"),
            Self::False => serializer.serialize_bool(false),
        }
    }
}

/// Construct a heat map trace.
///
/// # Examples
///
/// ```
/// use plotly::HeatMap;
///
/// let trace = HeatMap::new(
///     vec![0.0, 1.0],
///     vec![2.0, 3.0],
///     vec![vec![0.25, 0.75], vec![0.0, 0.5]]
/// );
///
/// let expected = serde_json::json!({
///     "type": "heatmap",
///     "x": [0.0, 1.0],
///     "y": [2.0, 3.0],
///     "z": [[0.25, 0.75], [0.0, 0.5]]
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct HeatMap<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    #[field_setter(default = "PlotType::HeatMap")]
    r#type: PlotType,
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(rename = "connectgaps")]
    connect_gaps: Option<bool>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hoverongaps")]
    hover_on_gaps: Option<bool>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Vec<String>>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    name: Option<String>,
    opacity: Option<f64>,
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
    text: Option<Vec<String>>,
    transpose: Option<bool>,
    visible: Option<Visible>,
    x: Option<Vec<X>>,
    #[serde(rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    y: Option<Vec<Y>>,
    #[serde(rename = "yaxis")]
    y_axis: Option<String>,
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
    z: Option<Vec<Z>>,
    zauto: Option<bool>,
    #[serde(rename = "zhoverformat")]
    zhover_format: Option<String>,
    zmax: Option<Z>,
    zmid: Option<Z>,
    zmin: Option<Z>,
    zsmooth: Option<Smoothing>,
}

impl<Z> HeatMap<f64, f64, Z>
where
    Z: Serialize + Clone,
{
    pub fn new_z(z: Vec<Z>) -> Box<Self> {
        Box::new(Self {
            z: Some(z),
            ..Default::default()
        })
    }
}

impl<X, Y, Z> HeatMap<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    pub fn new(x: Vec<X>, y: Vec<Y>, z: Vec<Z>) -> Box<Self> {
        Box::new(Self {
            x: Some(x),
            y: Some(y),
            z: Some(z),
            ..Default::default()
        })
    }
}

impl<X, Y, Z> Trace for HeatMap<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::ColorScalePalette;

    #[test]
    fn test_serialize_smoothing() {
        assert_eq!(to_value(Smoothing::Fast).unwrap(), json!("fast"));
        assert_eq!(to_value(Smoothing::Best).unwrap(), json!("best"));
        assert_eq!(to_value(Smoothing::False).unwrap(), json!(false));
    }

    #[test]
    fn test_serialize_default_heat_map() {
        let trace = HeatMap::<f64, f64, f64>::default();
        let expected = json!({"type": "heatmap"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn test_serialize_heat_map_z() {
        let trace = HeatMap::new_z(vec![1.0]);
        let expected = json!({
            "type": "heatmap",
            "z": [1.0],
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_heat_map() {
        let trace = HeatMap::new(vec![0.0, 1.0], vec![2.0, 3.0], vec![4.0, 5.0])
            .auto_color_scale(true)
            .color_bar(ColorBar::new())
            .color_scale(ColorScale::Palette(ColorScalePalette::Picnic))
            .connect_gaps(false)
            .hover_info(HoverInfo::None)
            .hover_label(Label::new())
            .hover_on_gaps(true)
            .hover_template("tmpl")
            .hover_template_array(vec!["tmpl1", "tmpl2"])
            .hover_text(vec!["hov", "er"])
            .legend_group("1")
            .legend_group_title("Legend Group Title")
            .name("name")
            .opacity(0.99)
            .reverse_scale(false)
            .show_legend(true)
            .show_scale(false)
            .text(vec!["te", "xt"])
            .transpose(true)
            .visible(Visible::LegendOnly)
            .x_axis("x")
            .x_calendar(Calendar::Hebrew)
            .y_axis("y")
            .y_calendar(Calendar::Islamic)
            .zauto(true)
            .zhover_format("fmt")
            .zmax(10.0)
            .zmid(5.0)
            .zmin(0.0)
            .zsmooth(Smoothing::Fast);

        let expected = json!({
            "type": "heatmap",
            "autocolorscale": true,
            "colorbar": {},
            "colorscale": "Picnic",
            "connectgaps": false,
            "hoverinfo": "none",
            "hoverlabel": {},
            "hoverongaps": true,
            "hovertemplate": ["tmpl1", "tmpl2"],
            "hovertext": ["hov", "er"],
            "legendgroup": "1",
            "legendgrouptitle": {"text": "Legend Group Title"},
            "name": "name",
            "opacity": 0.99,
            "reversescale": false,
            "showlegend": true,
            "showscale": false,
            "text": ["te", "xt"],
            "transpose": true,
            "visible": "legendonly",
            "x": [0.0, 1.0],
            "xcalendar": "hebrew",
            "xaxis": "x",
            "y": [2.0, 3.0],
            "yaxis": "y",
            "ycalendar": "islamic",
            "z": [4.0, 5.0],
            "zauto": true,
            "zhoverformat": "fmt",
            "zmax": 10.0,
            "zmid": 5.0,
            "zmin": 0.0,
            "zsmooth": "fast"
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
