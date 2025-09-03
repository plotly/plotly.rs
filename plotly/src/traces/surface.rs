//! Surface trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{
        Calendar, ColorBar, ColorScale, Dim, HoverInfo, Label, LegendGroupTitle, PlotType, Visible,
    },
    Trace,
};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Lighting {
    ambient: Option<f64>,
    diffuse: Option<f64>,
    fresnel: Option<f64>,
    roughness: Option<f64>,
    specular: Option<f64>,
}

impl Lighting {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, FieldSetter, Clone)]
pub struct PlaneProject {
    x: Option<bool>,
    y: Option<bool>,
    z: Option<bool>,
}

impl PlaneProject {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, FieldSetter, Clone)]
pub struct PlaneContours {
    color: Option<Box<dyn Color>>,
    end: Option<f64>,
    highlight: Option<bool>,
    #[serde(rename = "highlightwidth")]
    highlight_width: Option<usize>,
    #[serde(rename = "highlightcolor")]
    highlight_color: Option<Box<dyn Color>>,
    project: Option<PlaneProject>,
    show: Option<bool>,
    size: Option<usize>,
    start: Option<f64>,
    #[serde(rename = "usecolormap")]
    use_colormap: Option<bool>,
    width: Option<usize>,
}

impl PlaneContours {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, FieldSetter, Clone)]
pub struct SurfaceContours {
    x: Option<PlaneContours>,
    y: Option<PlaneContours>,
    z: Option<PlaneContours>,
}

impl SurfaceContours {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Construct a surface trace.
///
/// # Examples
///
/// ```
/// use plotly::Surface;
///
/// let trace = Surface::new(vec![vec![0, 1]]).x(vec![1, 2]).y(vec![2, 3]);
/// let expected = serde_json::json!({
///     "type": "surface",
///     "x": [1, 2],
///     "y": [2, 3],
///     "z": [[0, 1]],
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Surface<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Surface")]
    r#type: PlotType,
    x: Option<Vec<X>>,
    y: Option<Vec<Y>>,
    z: Option<Vec<Vec<Z>>>,
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    cauto: Option<bool>,
    cmax: Option<f64>,
    cmid: Option<f64>,
    cmin: Option<f64>,
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(rename = "connectgaps")]
    connect_gaps: Option<bool>,
    contours: Option<SurfaceContours>,
    #[serde(rename = "hidesurface")]
    hide_surface: Option<bool>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    #[serde(rename = "lightposition")]
    light_position: Option<Position>,
    lighting: Option<Lighting>,
    name: Option<String>,
    opacity: Option<f64>,
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(rename = "surfacecolor")]
    surface_color: Option<Vec<Box<dyn Color>>>,
    text: Option<Dim<String>>,
    visible: Option<Visible>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
    #[serde(rename = "zcalendar")]
    z_calendar: Option<Calendar>,
}

impl<X, Y, Z> Surface<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    pub fn new(z: Vec<Vec<Z>>) -> Box<Self> {
        Box::new(Self {
            z: Some(z),
            ..Default::default()
        })
    }
}

impl<X, Y, Z> Trace for Surface<X, Y, Z>
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
    fn serialize_lighting() {
        let lighting = Lighting::new()
            .ambient(0.0)
            .diffuse(1.0)
            .fresnel(2.0)
            .roughness(3.0)
            .specular(4.0);

        let expected = json!({
            "ambient": 0.0,
            "diffuse": 1.0,
            "fresnel": 2.0,
            "roughness": 3.0,
            "specular": 4.0,
        });

        assert_eq!(to_value(lighting).unwrap(), expected);
    }

    #[test]
    fn serialize_position() {
        let position = Position::new(0, 1, 2);
        let expected = json!({
            "x": 0,
            "y": 1,
            "z": 2,
        });

        assert_eq!(to_value(position).unwrap(), expected);
    }

    #[test]
    fn serialize_plane_project() {
        let plane_project = PlaneProject::new().x(true).y(false).z(true);
        let expected = json!({
            "x": true,
            "y": false,
            "z": true,
        });

        assert_eq!(to_value(plane_project).unwrap(), expected);
    }

    #[test]
    fn serialize_plane_contours() {
        let plane_contours = PlaneContours::new()
            .color("#123456")
            .highlight(true)
            .highlight_color("#456789")
            .highlight_width(5)
            .end(10.0)
            .project(PlaneProject::new().x(false).y(true).z(false))
            .show(false)
            .size(50)
            .start(0.0)
            .use_colormap(true)
            .width(100);

        let expected = json!({
            "color": "#123456",
            "highlight": true,
            "highlightcolor": "#456789",
            "highlightwidth": 5,
            "end": 10.0,
            "project": {"x": false, "y": true, "z": false},
            "show": false,
            "size": 50,
            "start": 0.0,
            "usecolormap": true,
            "width": 100
        });

        assert_eq!(to_value(plane_contours).unwrap(), expected);
    }

    #[test]
    fn serialize_surface_contours() {
        let surface_contours = SurfaceContours::new()
            .x(PlaneContours::new())
            .y(PlaneContours::new())
            .z(PlaneContours::new());

        let expected = json!({
            "x": {},
            "y": {},
            "z": {},
        });

        assert_eq!(to_value(surface_contours).unwrap(), expected);
    }

    #[test]
    fn serialize_default_surface() {
        let trace = Surface::<i32, i32, i32>::default();
        let expected = json!({"type": "surface"});

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn serialize_surface() {
        let trace = Surface::new(vec![vec![0, 1]])
            .x(vec![2, 3])
            .y(vec![4, 5])
            .auto_color_scale(true)
            .cauto(false)
            .cmax(5.0)
            .cmid(2.5)
            .cmin(0.0)
            .color_bar(ColorBar::new())
            .color_scale(ColorScale::Palette(ColorScalePalette::Blues))
            .connect_gaps(true)
            .contours(SurfaceContours::new())
            .hide_surface(false)
            .hover_info(HoverInfo::All)
            .hover_label(Label::new())
            .hover_template("hover_template")
            .hover_template_array(vec!["hover_template_1"])
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text_1"])
            .legend_group("legend_group")
            .legend_group_title("Legend Group Title")
            .lighting(Lighting::new())
            .light_position(Position::new(0, 0, 0))
            .name("surface_trace")
            .opacity(0.5)
            .reverse_scale(true)
            .surface_color(vec!["#123456"])
            .show_legend(true)
            .show_scale(false)
            .text("text")
            .text_array(vec!["text1", "text2"])
            .visible(Visible::False)
            .x_calendar(Calendar::Chinese)
            .y_calendar(Calendar::Coptic)
            .z_calendar(Calendar::DiscWorld);

        let expected = json!({
            "type": "surface",
            "x": [2, 3],
            "y": [4, 5],
            "z": [[0, 1]],
            "autocolorscale": true,
            "cauto": false,
            "cmax": 5.0,
            "cmid": 2.5,
            "cmin": 0.0,
            "colorbar": {},
            "colorscale": "Blues",
            "connectgaps": true,
            "contours": {},
            "hidesurface": false,
            "hoverinfo": "all",
            "hoverlabel": {},
            "hovertemplate": ["hover_template_1"],
            "hovertext": ["hover_text_1"],
            "legendgroup": "legend_group",
            "legendgrouptitle": {"text": "Legend Group Title"},
            "lighting": {},
            "lightposition": {"x": 0, "y": 0, "z": 0},
            "name": "surface_trace",
            "opacity": 0.5,
            "reversescale": true,
            "surfacecolor": ["#123456"],
            "showlegend": true,
            "showscale": false,
            "text": ["text1", "text2"],
            "visible": false,
            "xcalendar": "chinese",
            "ycalendar": "coptic",
            "zcalendar": "discworld",
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
