//! Volume (3D volume rendering) trace

use plotly_derive::FieldSetter;
use serde::{Serialize, Serializer};

use crate::{
    common::{ColorBar, ColorScale, Dim, Label, LegendGroupTitle, PlotType, Visible},
    private::{NumOrString, NumOrStringCollection},
    traces::{
        isosurface::{Caps, IsoSurfaceSurface, Slices, SpaceFrame},
        mesh3d::{Contour, LightPosition, Lighting},
    },
    Trace,
};

/// Maps normalized values of a [`Volume`] trace to opacities.
#[derive(Clone, Debug)]
pub enum OpacityScale {
    /// Same opacity for every value (the default).
    Uniform,
    /// Lower values are more opaque.
    Min,
    /// Higher values are more opaque.
    Max,
    /// The lowest and highest values are more opaque.
    Extremes,
    /// Custom `(normalized value, opacity)` pairs. Mappings for at least the
    /// lowest (0.0) and highest (1.0) values are required, e.g.
    /// `vec![(0.0, 1.0), (0.5, 0.2), (1.0, 1.0)]`.
    Custom(Vec<(f64, f64)>),
}

impl Serialize for OpacityScale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Uniform => serializer.serialize_str("uniform"),
            Self::Min => serializer.serialize_str("min"),
            Self::Max => serializer.serialize_str("max"),
            Self::Extremes => serializer.serialize_str("extremes"),
            Self::Custom(pairs) => pairs
                .iter()
                .map(|(value, opacity)| [*value, *opacity])
                .collect::<Vec<_>>()
                .serialize(serializer),
        }
    }
}

/// Construct a volume trace: a 3D volume rendering of `value` sampled on the
/// `(x, y, z)` grid, drawn as many semi-transparent isosurfaces between
/// `isomin` and `isomax`.
///
/// # Examples
///
/// ```
/// use plotly::{volume::OpacityScale, Volume};
///
/// let trace = Volume::new(
///     vec![0, 0, 1, 1], vec![0, 1, 0, 1], vec![1, 1, 1, 1], vec![1.0, 2.0, 3.0, 4.0],
/// )
/// .opacity(0.1)
/// .opacity_scale(OpacityScale::Extremes);
///
/// let expected = serde_json::json!({
///     "type": "volume",
///     "x": [0, 0, 1, 1],
///     "y": [0, 1, 0, 1],
///     "z": [1, 1, 1, 1],
///     "value": [1.0, 2.0, 3.0, 4.0],
///     "opacity": 0.1,
///     "opacityscale": "extremes",
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Volume<X, Y, Z, V>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
    V: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Volume")]
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
    /// Sets the opacity of the surface. Volume renderings typically need a
    /// low opacity (e.g. 0.1) to see inside the volume.
    opacity: Option<f64>,
    #[serde(rename = "opacityscale")]
    opacity_scale: Option<OpacityScale>,
    ids: Option<Vec<String>>,
    /// Sets the x coordinates of the vertices on the x axis.
    x: Option<Vec<X>>,
    /// Sets the y coordinates of the vertices on the y axis.
    y: Option<Vec<Y>>,
    /// Sets the z coordinates of the vertices on the z axis.
    z: Option<Vec<Z>>,
    /// Sets the 4th dimension (value) of the vertices.
    value: Option<Vec<V>>,
    /// Sets the minimum boundary of the rendered values.
    #[serde(rename = "isomin")]
    iso_min: Option<f64>,
    /// Sets the maximum boundary of the rendered values.
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

impl<X, Y, Z, V> Volume<X, Y, Z, V>
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

impl<X, Y, Z, V> Trace for Volume<X, Y, Z, V>
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
    fn serialize_opacity_scale() {
        assert_eq!(to_value(OpacityScale::Uniform).unwrap(), json!("uniform"));
        assert_eq!(to_value(OpacityScale::Min).unwrap(), json!("min"));
        assert_eq!(to_value(OpacityScale::Max).unwrap(), json!("max"));
        assert_eq!(to_value(OpacityScale::Extremes).unwrap(), json!("extremes"));
        assert_eq!(
            to_value(OpacityScale::Custom(vec![
                (0.0, 1.0),
                (0.5, 0.2),
                (1.0, 1.0)
            ]))
            .unwrap(),
            json!([[0.0, 1.0], [0.5, 0.2], [1.0, 1.0]])
        );
    }

    #[test]
    fn serialize_volume() {
        let trace = Volume::new(vec![0], vec![1], vec![2], vec![3.0])
            .name("vol")
            .visible(Visible::True)
            .show_legend(true)
            .legend_group("g")
            .legend_group_title("G")
            .legend_rank(1)
            .legend_width(2.0)
            .opacity(0.1)
            .opacity_scale(OpacityScale::Max)
            .ids(vec!["a"])
            .iso_min(0.0)
            .iso_max(5.0)
            .surface(IsoSurfaceSurface::new().count(17))
            .caps(Caps::new())
            .slices(Slices::new())
            .space_frame(SpaceFrame::new())
            .contour(Contour::new())
            .flat_shading(true)
            .scene("scene")
            .text("t")
            .hover_text("h")
            .hover_info("all")
            .hover_template("%{value}")
            .hover_template_fallback("-")
            .hover_label(Label::new())
            .x_hover_format("a")
            .y_hover_format("b")
            .z_hover_format("c")
            .value_hover_format("d")
            .color_axis("coloraxis")
            .color_bar(ColorBar::new())
            .color_scale(ColorScale::Palette(ColorScalePalette::Jet))
            .auto_color_scale(false)
            .reverse_scale(false)
            .show_scale(true)
            .c_auto(true)
            .c_min(0.0)
            .c_max(5.0)
            .c_mid(2.5)
            .lighting(Lighting::new())
            .light_position(LightPosition::new())
            .meta("m")
            .custom_data(vec!["cd"])
            .ui_revision(1);

        let expected = json!({
            "type": "volume",
            "name": "vol",
            "visible": true,
            "showlegend": true,
            "legendgroup": "g",
            "legendgrouptitle": {"text": "G"},
            "legendrank": 1,
            "legendwidth": 2.0,
            "opacity": 0.1,
            "opacityscale": "max",
            "ids": ["a"],
            "x": [0], "y": [1], "z": [2],
            "value": [3.0],
            "isomin": 0.0,
            "isomax": 5.0,
            "surface": {"count": 17},
            "caps": {},
            "slices": {},
            "spaceframe": {},
            "contour": {},
            "flatshading": true,
            "scene": "scene",
            "text": "t",
            "hovertext": "h",
            "hoverinfo": "all",
            "hovertemplate": "%{value}",
            "hovertemplatefallback": "-",
            "hoverlabel": {},
            "xhoverformat": "a",
            "yhoverformat": "b",
            "zhoverformat": "c",
            "valuehoverformat": "d",
            "coloraxis": "coloraxis",
            "colorbar": {},
            "colorscale": "Jet",
            "autocolorscale": false,
            "reversescale": false,
            "showscale": true,
            "cauto": true,
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
