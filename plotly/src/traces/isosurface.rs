//! Isosurface (3D volume isosurface) trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    common::{ColorBar, ColorScale, Dim, Label, LegendGroupTitle, PlotType, Visible},
    private::{NumOrString, NumOrStringCollection},
    traces::mesh3d::{Contour, LightPosition, Lighting},
    Trace,
};

/// Settings for the caps along one axis of an [`IsoSurface`] or
/// [`Volume`](crate::Volume) trace.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Cap {
    /// Sets the fill ratio of the caps (between 0.0 and 1.0). The default
    /// fill value of 1 makes the caps solid.
    fill: Option<f64>,
    /// Sets whether the caps are drawn along this axis.
    show: Option<bool>,
}

impl Cap {
    pub fn new() -> Self {
        Default::default()
    }
}

/// The caps (surfaces closing off the volume at the domain edges) of an
/// [`IsoSurface`] or [`Volume`](crate::Volume) trace.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Caps {
    x: Option<Cap>,
    y: Option<Cap>,
    z: Option<Cap>,
}

impl Caps {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Slices along one axis of an [`IsoSurface`] or [`Volume`](crate::Volume)
/// trace.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Slice {
    /// Sets the fill ratio of the slices (between 0.0 and 1.0).
    fill: Option<f64>,
    /// Specifies the locations of the slices on this axis. When unset,
    /// slices are made at every point of the axis.
    locations: Option<Vec<f64>>,
    show: Option<bool>,
}

impl Slice {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Slices through an [`IsoSurface`] or [`Volume`](crate::Volume) trace.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Slices {
    x: Option<Slice>,
    y: Option<Slice>,
    z: Option<Slice>,
}

impl Slices {
    pub fn new() -> Self {
        Default::default()
    }
}

/// The space frame (the volume between `isomin` and `isomax`, drawn as
/// tetrahedra) of an [`IsoSurface`] or [`Volume`](crate::Volume) trace.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct SpaceFrame {
    /// Sets the fill ratio of the space frame elements (between 0.0 and 1.0).
    fill: Option<f64>,
    show: Option<bool>,
}

impl SpaceFrame {
    pub fn new() -> Self {
        Default::default()
    }
}

/// The isosurfaces of an [`IsoSurface`] or [`Volume`](crate::Volume) trace.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct IsoSurfaceSurface {
    /// Sets the number of isosurfaces between `isomin` and `isomax`.
    count: Option<usize>,
    /// Sets the fill ratio of the isosurface (between 0.0 and 1.0).
    fill: Option<f64>,
    /// Sets the surface pattern of the isosurface 3-D sections: a flaglist of
    /// `A`, `B`, `C`, `D`, `E` joined with `+`, or one of `all`, `odd`,
    /// `even`.
    pattern: Option<String>,
    show: Option<bool>,
}

impl IsoSurfaceSurface {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Construct an isosurface trace: surfaces of constant `value` through a 3D
/// volume sampled on the `(x, y, z)` grid, between `isomin` and `isomax`.
///
/// # Examples
///
/// ```
/// use plotly::IsoSurface;
///
/// let trace = IsoSurface::new(
///     vec![0, 0, 1, 1], vec![0, 1, 0, 1], vec![1, 1, 1, 1], vec![1.0, 2.0, 3.0, 4.0],
/// )
/// .iso_min(1.5)
/// .iso_max(3.5);
///
/// let expected = serde_json::json!({
///     "type": "isosurface",
///     "x": [0, 0, 1, 1],
///     "y": [0, 1, 0, 1],
///     "z": [1, 1, 1, 1],
///     "value": [1.0, 2.0, 3.0, 4.0],
///     "isomin": 1.5,
///     "isomax": 3.5,
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct IsoSurface<X, Y, Z, V>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
    V: Serialize + Clone,
{
    #[field_setter(default = "PlotType::IsoSurface")]
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
    /// Sets the x coordinates of the vertices on the x axis.
    x: Option<Vec<X>>,
    /// Sets the y coordinates of the vertices on the y axis.
    y: Option<Vec<Y>>,
    /// Sets the z coordinates of the vertices on the z axis.
    z: Option<Vec<Z>>,
    /// Sets the 4th dimension (value) of the vertices.
    value: Option<Vec<V>>,
    /// Sets the minimum boundary for the iso-surface plot.
    #[serde(rename = "isomin")]
    iso_min: Option<f64>,
    /// Sets the maximum boundary for the iso-surface plot.
    #[serde(rename = "isomax")]
    iso_max: Option<f64>,
    surface: Option<IsoSurfaceSurface>,
    caps: Option<Caps>,
    slices: Option<Slices>,
    #[serde(rename = "spaceframe")]
    space_frame: Option<SpaceFrame>,
    contour: Option<Contour>,
    #[serde(rename = "flatshading")]
    flat_shading: Option<bool>,
    /// Sets a reference between this trace's 3D coordinate system and a 3D
    /// scene, e.g. `"scene"` (the default) or `"scene2"`.
    scene: Option<String>,
    text: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Flaglist of `x`, `y`, `z`, `text`, `name` joined with `+`, or one of
    /// `all`, `none`, `skip`.
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
    #[serde(rename = "valuehoverformat")]
    value_hover_format: Option<String>,
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

impl<X, Y, Z, V> IsoSurface<X, Y, Z, V>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
    V: Serialize + Clone,
{
    pub fn new(x: Vec<X>, y: Vec<Y>, z: Vec<Z>, value: Vec<V>) -> Box<Self> {
        Box::new(Self {
            x: Some(x),
            y: Some(y),
            z: Some(z),
            value: Some(value),
            ..Default::default()
        })
    }
}

impl<X, Y, Z, V> Trace for IsoSurface<X, Y, Z, V>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
    V: Serialize + Clone,
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
    fn serialize_sub_structs() {
        let caps = Caps::new()
            .x(Cap::new().fill(0.5).show(true))
            .y(Cap::new().show(false))
            .z(Cap::new());
        assert_eq!(
            to_value(caps).unwrap(),
            json!({"x": {"fill": 0.5, "show": true}, "y": {"show": false}, "z": {}})
        );

        let slices = Slices::new()
            .x(Slice::new().fill(1.0).locations(vec![0.5]).show(true))
            .y(Slice::new())
            .z(Slice::new());
        assert_eq!(
            to_value(slices).unwrap(),
            json!({"x": {"fill": 1.0, "locations": [0.5], "show": true}, "y": {}, "z": {}})
        );

        let space_frame = SpaceFrame::new().fill(0.15).show(true);
        assert_eq!(
            to_value(space_frame).unwrap(),
            json!({"fill": 0.15, "show": true})
        );

        let surface = IsoSurfaceSurface::new()
            .count(3)
            .fill(0.8)
            .pattern("A+B")
            .show(true);
        assert_eq!(
            to_value(surface).unwrap(),
            json!({"count": 3, "fill": 0.8, "pattern": "A+B", "show": true})
        );
    }

    #[test]
    fn serialize_iso_surface() {
        let trace = IsoSurface::new(vec![0], vec![1], vec![2], vec![3.0])
            .name("iso")
            .visible(Visible::True)
            .show_legend(true)
            .legend_group("g")
            .legend_group_title("G")
            .legend_rank(1)
            .legend_width(2.0)
            .opacity(0.6)
            .ids(vec!["a"])
            .iso_min(0.0)
            .iso_max(5.0)
            .surface(IsoSurfaceSurface::new().count(2))
            .caps(Caps::new())
            .slices(Slices::new())
            .space_frame(SpaceFrame::new())
            .contour(Contour::new().show(true))
            .flat_shading(false)
            .scene("scene3")
            .text("t")
            .hover_text("h")
            .hover_info("x+y+z")
            .hover_template("%{value}")
            .hover_template_fallback("-")
            .hover_label(Label::new())
            .x_hover_format("a")
            .y_hover_format("b")
            .z_hover_format("c")
            .value_hover_format("d")
            .color_axis("coloraxis")
            .color_bar(ColorBar::new())
            .color_scale(ColorScale::Palette(ColorScalePalette::Blues))
            .auto_color_scale(false)
            .reverse_scale(true)
            .show_scale(false)
            .c_auto(false)
            .c_min(0.0)
            .c_max(5.0)
            .c_mid(2.5)
            .lighting(Lighting::new())
            .light_position(LightPosition::new())
            .meta("m")
            .custom_data(vec!["cd"])
            .ui_revision(1);

        let expected = json!({
            "type": "isosurface",
            "name": "iso",
            "visible": true,
            "showlegend": true,
            "legendgroup": "g",
            "legendgrouptitle": {"text": "G"},
            "legendrank": 1,
            "legendwidth": 2.0,
            "opacity": 0.6,
            "ids": ["a"],
            "x": [0], "y": [1], "z": [2],
            "value": [3.0],
            "isomin": 0.0,
            "isomax": 5.0,
            "surface": {"count": 2},
            "caps": {},
            "slices": {},
            "spaceframe": {},
            "contour": {"show": true},
            "flatshading": false,
            "scene": "scene3",
            "text": "t",
            "hovertext": "h",
            "hoverinfo": "x+y+z",
            "hovertemplate": "%{value}",
            "hovertemplatefallback": "-",
            "hoverlabel": {},
            "xhoverformat": "a",
            "yhoverformat": "b",
            "zhoverformat": "c",
            "valuehoverformat": "d",
            "coloraxis": "coloraxis",
            "colorbar": {},
            "colorscale": "Blues",
            "autocolorscale": false,
            "reversescale": true,
            "showscale": false,
            "cauto": false,
            "cmin": 0.0,
            "cmax": 5.0,
            "cmid": 2.5,
            "lighting": {},
            "lightposition": {},
            "meta": "m",
            "customdata": ["cd"],
            "uirevision": 1,
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
