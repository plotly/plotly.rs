//! Surface trace

use serde::Serialize;

use crate::{
    color::{Color, ColorArray},
    common::{Calendar, ColorBar, ColorScale, Dim, HoverInfo, Label, PlotType, Visible},
    private, Trace,
};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
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

    pub fn ambient(mut self, ambient: f64) -> Self {
        self.ambient = Some(ambient);
        self
    }

    pub fn diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = Some(diffuse);
        self
    }

    pub fn fresnel(mut self, fresnel: f64) -> Self {
        self.fresnel = Some(fresnel);
        self
    }

    pub fn roughness(mut self, roughness: f64) -> Self {
        self.roughness = Some(roughness);
        self
    }

    pub fn specular(mut self, specular: f64) -> Self {
        self.specular = Some(specular);
        self
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
#[derive(Serialize, Debug, Default, Clone)]
pub struct PlaneProject {
    x: Option<bool>,
    y: Option<bool>,
    z: Option<bool>,
}

impl PlaneProject {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn x(mut self, x: bool) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: bool) -> Self {
        self.y = Some(y);
        self
    }

    pub fn z(mut self, z: bool) -> Self {
        self.z = Some(z);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
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

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Box::new(color));
        self
    }

    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = Some(highlight);
        self
    }

    pub fn highlight_color<C: Color>(mut self, highlight_color: C) -> Self {
        self.highlight_color = Some(Box::new(highlight_color));
        self
    }

    pub fn highlight_width(mut self, highlight_width: usize) -> Self {
        self.highlight_width = Some(highlight_width);
        self
    }

    pub fn end(mut self, end: f64) -> Self {
        self.end = Some(end);
        self
    }

    pub fn project(mut self, project: PlaneProject) -> Self {
        self.project = Some(project);
        self
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn start(mut self, start: f64) -> Self {
        self.start = Some(start);
        self
    }

    pub fn use_colormap(mut self, use_colormap: bool) -> Self {
        self.use_colormap = Some(use_colormap);
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
pub struct SurfaceContours {
    x: Option<PlaneContours>,
    y: Option<PlaneContours>,
    z: Option<PlaneContours>,
}

impl SurfaceContours {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn x(mut self, x: PlaneContours) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: PlaneContours) -> Self {
        self.y = Some(y);
        self
    }

    pub fn z(mut self, z: PlaneContours) -> Self {
        self.z = Some(z);
        self
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
#[derive(Serialize, Debug, Clone)]
pub struct Surface<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
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

impl<X, Y, Z> Default for Surface<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    fn default() -> Self {
        Self {
            r#type: PlotType::Surface,
            x: None,
            y: None,
            z: None,
            auto_color_scale: None,
            cauto: None,
            cmax: None,
            cmid: None,
            cmin: None,
            color_bar: None,
            color_scale: None,
            connect_gaps: None,
            contours: None,
            hide_surface: None,
            hover_info: None,
            hover_label: None,
            hover_template: None,
            hover_text: None,
            legend_group: None,
            light_position: None,
            lighting: None,
            name: None,
            opacity: None,
            reverse_scale: None,
            show_legend: None,
            show_scale: None,
            surface_color: None,
            text: None,
            visible: None,
            x_calendar: None,
            y_calendar: None,
            z_calendar: None,
        }
    }
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

    pub fn x(mut self, x: Vec<X>) -> Box<Self> {
        self.x = Some(x);
        Box::new(self)
    }

    pub fn y(mut self, y: Vec<Y>) -> Box<Self> {
        self.y = Some(y);
        Box::new(self)
    }

    pub fn name(mut self, name: &str) -> Box<Self> {
        self.name = Some(name.to_string());
        Box::new(self)
    }

    pub fn visible(mut self, visible: Visible) -> Box<Self> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Self> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Self> {
        self.legend_group = Some(legend_group.to_string());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Self> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn surface_color<C: Color>(mut self, surface_color: Vec<C>) -> Box<Self> {
        self.surface_color = Some(ColorArray(surface_color).into());
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Self> {
        self.text = Some(Dim::Scalar(text.to_string()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Self> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Self> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_string()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Self> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Self> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<Self> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_string()));
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(mut self, hover_template: Vec<S>) -> Box<Self> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn color_bar(mut self, color_bar: ColorBar) -> Box<Self> {
        self.color_bar = Some(color_bar);
        Box::new(self)
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> Box<Self> {
        self.auto_color_scale = Some(auto_color_scale);
        Box::new(self)
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> Box<Self> {
        self.color_scale = Some(color_scale);
        Box::new(self)
    }

    pub fn show_scale(mut self, show_scale: bool) -> Box<Self> {
        self.show_scale = Some(show_scale);
        Box::new(self)
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> Box<Self> {
        self.reverse_scale = Some(reverse_scale);
        Box::new(self)
    }

    pub fn cauto(mut self, cauto: bool) -> Box<Self> {
        self.cauto = Some(cauto);
        Box::new(self)
    }

    pub fn cmin(mut self, cmin: f64) -> Box<Self> {
        self.cmin = Some(cmin);
        Box::new(self)
    }

    pub fn cmax(mut self, cmax: f64) -> Box<Self> {
        self.cmax = Some(cmax);
        Box::new(self)
    }

    pub fn cmid(mut self, cmid: f64) -> Box<Self> {
        self.cmid = Some(cmid);
        Box::new(self)
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> Box<Self> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }

    pub fn contours(mut self, contours: SurfaceContours) -> Box<Self> {
        self.contours = Some(contours);
        Box::new(self)
    }

    pub fn hide_surface(mut self, hide_surface: bool) -> Box<Self> {
        self.hide_surface = Some(hide_surface);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Self> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn lighting(mut self, lighting: Lighting) -> Box<Self> {
        self.lighting = Some(lighting);
        Box::new(self)
    }

    pub fn light_position(mut self, light_position: Position) -> Box<Self> {
        self.light_position = Some(light_position);
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Self> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Self> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }

    pub fn z_calendar(mut self, z_calendar: Calendar) -> Box<Self> {
        self.z_calendar = Some(z_calendar);
        Box::new(self)
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
    fn test_serialize_lighting() {
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
    fn test_serialize_position() {
        let position = Position::new(0, 1, 2);
        let expected = json!({
            "x": 0,
            "y": 1,
            "z": 2,
        });

        assert_eq!(to_value(position).unwrap(), expected);
    }

    #[test]
    fn test_serialize_plane_project() {
        let plane_project = PlaneProject::new().x(true).y(false).z(true);
        let expected = json!({
            "x": true,
            "y": false,
            "z": true,
        });

        assert_eq!(to_value(plane_project).unwrap(), expected);
    }

    #[test]
    fn test_serialize_plane_contours() {
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
    fn test_serialize_surface_contours() {
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
    fn test_serialize_default_surface() {
        let trace = Surface::<i32, i32, i32>::default();
        let expected = json!({"type": "surface"});

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_surface() {
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
