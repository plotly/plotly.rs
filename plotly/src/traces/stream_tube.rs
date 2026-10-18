//! Streamtube (3D flow) trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    common::{ColorBar, ColorScale, Dim, Label, LegendGroupTitle, PlotType, Visible},
    private::{NumOrString, NumOrStringCollection},
    traces::mesh3d::{LightPosition, Lighting},
    Trace,
};

/// Starting positions of the streamtubes. When unset, tubes start from the
/// `x`-`y`-`z` plane at the lowest `x`.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Starts {
    x: Option<Vec<f64>>,
    y: Option<Vec<f64>>,
    z: Option<Vec<f64>>,
}

impl Starts {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Construct a streamtube trace: tubes following the flow of a 3D vector
/// field `(u, v, w)` sampled on the `(x, y, z)` grid. The tube diameter
/// reflects the local divergence of the field.
///
/// # Examples
///
/// ```
/// use plotly::StreamTube;
///
/// let trace = StreamTube::new(
///     vec![0.0, 1.0], vec![0.0, 1.0], vec![0.0, 1.0],
///     vec![1.0, 1.0], vec![0.0, 0.0], vec![0.0, 0.0],
/// );
///
/// let expected = serde_json::json!({
///     "type": "streamtube",
///     "x": [0.0, 1.0], "y": [0.0, 1.0], "z": [0.0, 1.0],
///     "u": [1.0, 1.0], "v": [0.0, 0.0], "w": [0.0, 0.0],
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct StreamTube<X, Y, Z, U, V, W>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
    U: Serialize + Clone,
    V: Serialize + Clone,
    W: Serialize + Clone,
{
    #[field_setter(default = "PlotType::StreamTube")]
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    #[serde(rename = "legendwidth")]
    legend_width: Option<f64>,
    opacity: Option<f64>,
    ids: Option<Vec<String>>,
    /// Sets the x coordinates of the vector field.
    x: Option<Vec<X>>,
    /// Sets the y coordinates of the vector field.
    y: Option<Vec<Y>>,
    /// Sets the z coordinates of the vector field.
    z: Option<Vec<Z>>,
    /// Sets the x components of the vector field.
    u: Option<Vec<U>>,
    /// Sets the y components of the vector field.
    v: Option<Vec<V>>,
    /// Sets the z components of the vector field.
    w: Option<Vec<W>>,
    starts: Option<Starts>,
    /// The maximum number of displayed segments in a streamtube.
    #[serde(rename = "maxdisplayed")]
    max_displayed: Option<usize>,
    /// The scaling factor for the streamtubes. The default is 1, which avoids
    /// two max divergence tubes from touching at adjacent starting positions.
    #[serde(rename = "sizeref")]
    size_ref: Option<f64>,
    /// Sets a reference between this trace's 3D coordinate system and a 3D
    /// scene, e.g. `"scene"` (the default) or `"scene2"`.
    scene: Option<String>,
    /// Sets a text element associated with this trace, shown in hover labels
    /// when `hoverinfo` contains `text`.
    text: Option<String>,
    #[serde(rename = "hovertext")]
    hover_text: Option<String>,
    /// Flaglist of `x`, `y`, `z`, `u`, `v`, `w`, `norm`, `divergence`,
    /// `text`, `name` joined with `+`, or one of `all`, `none`, `skip`.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<String>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertemplatefallback")]
    hover_template_fallback: Option<Dim<String>>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "xhoverformat")]
    x_hover_format: Option<String>,
    #[serde(rename = "yhoverformat")]
    y_hover_format: Option<String>,
    #[serde(rename = "zhoverformat")]
    z_hover_format: Option<String>,
    #[serde(rename = "uhoverformat")]
    u_hover_format: Option<String>,
    #[serde(rename = "vhoverformat")]
    v_hover_format: Option<String>,
    #[serde(rename = "whoverformat")]
    w_hover_format: Option<String>,
    #[serde(rename = "coloraxis")]
    color_axis: Option<String>,
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(rename = "cauto")]
    c_auto: Option<bool>,
    #[serde(rename = "cmin")]
    c_min: Option<f64>,
    #[serde(rename = "cmax")]
    c_max: Option<f64>,
    #[serde(rename = "cmid")]
    c_mid: Option<f64>,
    lighting: Option<Lighting>,
    #[serde(rename = "lightposition")]
    light_position: Option<LightPosition>,
    meta: Option<NumOrString>,
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<X, Y, Z, U, V, W> StreamTube<X, Y, Z, U, V, W>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
    U: Serialize + Clone,
    V: Serialize + Clone,
    W: Serialize + Clone,
{
    pub fn new(x: Vec<X>, y: Vec<Y>, z: Vec<Z>, u: Vec<U>, v: Vec<V>, w: Vec<W>) -> Box<Self> {
        Box::new(Self {
            x: Some(x),
            y: Some(y),
            z: Some(z),
            u: Some(u),
            v: Some(v),
            w: Some(w),
            ..Default::default()
        })
    }
}

impl<X, Y, Z, U, V, W> Trace for StreamTube<X, Y, Z, U, V, W>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
    U: Serialize + Clone,
    V: Serialize + Clone,
    W: Serialize + Clone,
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
    fn serialize_starts() {
        let starts = Starts::new().x(vec![0.0]).y(vec![1.0]).z(vec![2.0]);
        assert_eq!(
            to_value(starts).unwrap(),
            json!({"x": [0.0], "y": [1.0], "z": [2.0]})
        );
    }

    #[test]
    fn serialize_stream_tube() {
        let trace = StreamTube::new(vec![0], vec![1], vec![2], vec![1], vec![0], vec![0])
            .name("tube")
            .visible(Visible::LegendOnly)
            .show_legend(true)
            .legend_group("g")
            .legend_group_title("G")
            .legend_rank(1)
            .legend_width(2.0)
            .opacity(0.7)
            .ids(vec!["a"])
            .starts(Starts::new().x(vec![0.0]))
            .max_displayed(500)
            .size_ref(0.3)
            .scene("scene")
            .text("t")
            .hover_text("h")
            .hover_info("divergence+norm")
            .hover_template("%{divergence}")
            .hover_template_fallback("-")
            .hover_label(Label::new())
            .x_hover_format("a")
            .y_hover_format("b")
            .z_hover_format("c")
            .u_hover_format("d")
            .v_hover_format("e")
            .w_hover_format("f")
            .color_axis("coloraxis")
            .color_bar(ColorBar::new())
            .color_scale(ColorScale::Palette(ColorScalePalette::Portland))
            .auto_color_scale(true)
            .reverse_scale(false)
            .show_scale(true)
            .c_auto(true)
            .c_min(-1.0)
            .c_max(1.0)
            .c_mid(0.0)
            .lighting(Lighting::new())
            .light_position(LightPosition::new())
            .meta("m")
            .custom_data(vec!["cd"])
            .ui_revision("rev");

        let expected = json!({
            "type": "streamtube",
            "name": "tube",
            "visible": "legendonly",
            "showlegend": true,
            "legendgroup": "g",
            "legendgrouptitle": {"text": "G"},
            "legendrank": 1,
            "legendwidth": 2.0,
            "opacity": 0.7,
            "ids": ["a"],
            "x": [0], "y": [1], "z": [2],
            "u": [1], "v": [0], "w": [0],
            "starts": {"x": [0.0]},
            "maxdisplayed": 500,
            "sizeref": 0.3,
            "scene": "scene",
            "text": "t",
            "hovertext": "h",
            "hoverinfo": "divergence+norm",
            "hovertemplate": "%{divergence}",
            "hovertemplatefallback": "-",
            "hoverlabel": {},
            "xhoverformat": "a",
            "yhoverformat": "b",
            "zhoverformat": "c",
            "uhoverformat": "d",
            "vhoverformat": "e",
            "whoverformat": "f",
            "coloraxis": "coloraxis",
            "colorbar": {},
            "colorscale": "Portland",
            "autocolorscale": true,
            "reversescale": false,
            "showscale": true,
            "cauto": true,
            "cmin": -1.0,
            "cmax": 1.0,
            "cmid": 0.0,
            "lighting": {},
            "lightposition": {},
            "meta": "m",
            "customdata": ["cd"],
            "uirevision": "rev",
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
