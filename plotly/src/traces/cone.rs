//! Cone (3D vector field) trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    common::{ColorBar, ColorScale, Dim, Label, LegendGroupTitle, PlotType, Visible},
    private::{NumOrString, NumOrStringCollection},
    traces::mesh3d::{LightPosition, Lighting},
    Trace,
};

/// Sets the cones' anchor with respect to their x/y/z positions.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ConeAnchor {
    Tip,
    Tail,
    /// Center of mass.
    Cm,
    Center,
}

/// Determines whether `sizeref` is set as a scaling factor relative to the
/// vector field, as an absolute value, or whether the raw vector norms are
/// used.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ConeSizeMode {
    Scaled,
    Absolute,
    Raw,
}

/// Construct a cone trace: a 3D vector field where each cone sits at an
/// `(x, y, z)` position and points along the `(u, v, w)` vector.
///
/// # Examples
///
/// ```
/// use plotly::Cone;
///
/// let trace = Cone::new(
///     vec![1.0], vec![1.0], vec![1.0],
///     vec![1.0], vec![1.0], vec![0.0],
/// );
///
/// let expected = serde_json::json!({
///     "type": "cone",
///     "x": [1.0], "y": [1.0], "z": [1.0],
///     "u": [1.0], "v": [1.0], "w": [0.0],
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Cone<X, Y, Z, U, V, W>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
    U: Serialize + Clone,
    V: Serialize + Clone,
    W: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Cone")]
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
    /// Sets the x coordinates of the vector field and of the displayed cones.
    x: Option<Vec<X>>,
    /// Sets the y coordinates of the vector field and of the displayed cones.
    y: Option<Vec<Y>>,
    /// Sets the z coordinates of the vector field and of the displayed cones.
    z: Option<Vec<Z>>,
    /// Sets the x components of the vector field.
    u: Option<Vec<U>>,
    /// Sets the y components of the vector field.
    v: Option<Vec<V>>,
    /// Sets the z components of the vector field.
    w: Option<Vec<W>>,
    anchor: Option<ConeAnchor>,
    #[serde(rename = "sizemode")]
    size_mode: Option<ConeSizeMode>,
    /// Adjusts the cone size scaling. Its meaning depends on `sizemode`.
    #[serde(rename = "sizeref")]
    size_ref: Option<f64>,
    /// Sets a reference between this trace's 3D coordinate system and a 3D
    /// scene, e.g. `"scene"` (the default) or `"scene2"`.
    scene: Option<String>,
    text: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Flaglist of `x`, `y`, `z`, `u`, `v`, `w`, `norm`, `text`, `name`
    /// joined with `+`, or one of `all`, `none`, `skip`.
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

impl<X, Y, Z, U, V, W> Cone<X, Y, Z, U, V, W>
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

impl<X, Y, Z, U, V, W> Trace for Cone<X, Y, Z, U, V, W>
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
    fn serialize_enums() {
        assert_eq!(to_value(ConeAnchor::Tip).unwrap(), json!("tip"));
        assert_eq!(to_value(ConeAnchor::Tail).unwrap(), json!("tail"));
        assert_eq!(to_value(ConeAnchor::Cm).unwrap(), json!("cm"));
        assert_eq!(to_value(ConeAnchor::Center).unwrap(), json!("center"));
        assert_eq!(to_value(ConeSizeMode::Scaled).unwrap(), json!("scaled"));
        assert_eq!(to_value(ConeSizeMode::Absolute).unwrap(), json!("absolute"));
        assert_eq!(to_value(ConeSizeMode::Raw).unwrap(), json!("raw"));
    }

    #[test]
    fn serialize_cone() {
        let trace = Cone::new(vec![1], vec![2], vec![3], vec![0.1], vec![0.2], vec![0.3])
            .name("cone")
            .visible(Visible::True)
            .show_legend(true)
            .legend_group("g")
            .legend_group_title("G")
            .legend_rank(1)
            .legend_width(2.0)
            .opacity(0.9)
            .ids(vec!["a"])
            .anchor(ConeAnchor::Tip)
            .size_mode(ConeSizeMode::Absolute)
            .size_ref(0.5)
            .scene("scene2")
            .text("t")
            .hover_text("h")
            .hover_info("u+v+w+norm")
            .hover_template("%{u}")
            .hover_template_fallback("-")
            .hover_label(Label::new())
            .x_hover_format(".1f")
            .y_hover_format(".2f")
            .z_hover_format(".3f")
            .u_hover_format(".4f")
            .v_hover_format(".5f")
            .w_hover_format(".6f")
            .color_axis("coloraxis2")
            .color_bar(ColorBar::new())
            .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
            .auto_color_scale(false)
            .reverse_scale(true)
            .show_scale(false)
            .c_auto(false)
            .c_min(0.0)
            .c_max(1.0)
            .c_mid(0.5)
            .lighting(Lighting::new().ambient(0.5))
            .light_position(LightPosition::new().x(1.0).y(2.0).z(3.0))
            .meta("m")
            .custom_data(vec!["cd"])
            .ui_revision(3);

        let expected = json!({
            "type": "cone",
            "name": "cone",
            "visible": true,
            "showlegend": true,
            "legendgroup": "g",
            "legendgrouptitle": {"text": "G"},
            "legendrank": 1,
            "legendwidth": 2.0,
            "opacity": 0.9,
            "ids": ["a"],
            "x": [1], "y": [2], "z": [3],
            "u": [0.1], "v": [0.2], "w": [0.3],
            "anchor": "tip",
            "sizemode": "absolute",
            "sizeref": 0.5,
            "scene": "scene2",
            "text": "t",
            "hovertext": "h",
            "hoverinfo": "u+v+w+norm",
            "hovertemplate": "%{u}",
            "hovertemplatefallback": "-",
            "hoverlabel": {},
            "xhoverformat": ".1f",
            "yhoverformat": ".2f",
            "zhoverformat": ".3f",
            "uhoverformat": ".4f",
            "vhoverformat": ".5f",
            "whoverformat": ".6f",
            "coloraxis": "coloraxis2",
            "colorbar": {},
            "colorscale": "Viridis",
            "autocolorscale": false,
            "reversescale": true,
            "showscale": false,
            "cauto": false,
            "cmin": 0.0,
            "cmax": 1.0,
            "cmid": 0.5,
            "lighting": {"ambient": 0.5},
            "lightposition": {"x": 1.0, "y": 2.0, "z": 3.0},
            "meta": "m",
            "customdata": ["cd"],
            "uirevision": 3,
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
