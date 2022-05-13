//! Surface plot

use crate::common::color::{Color, ColorWrapper};
use crate::common::{Calendar, ColorBar, ColorScale, Dim, HoverInfo, Label, PlotType, Visible};
use crate::private;
use crate::Trace;
use serde::Serialize;

#[derive(Serialize, Debug, Default, Clone)]
pub struct Lighting {
    #[serde(skip_serializing_if = "Option::is_none")]
    ambient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diffuse: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specular: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roughness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fresnel: Option<f64>,
}

impl Lighting {
    pub fn new() -> Lighting {
        Default::default()
    }

    pub fn ambient(mut self, ambient: f64) -> Lighting {
        self.ambient = Some(ambient);
        self
    }

    pub fn diffuse(mut self, diffuse: f64) -> Lighting {
        self.diffuse = Some(diffuse);
        self
    }

    pub fn specular(mut self, specular: f64) -> Lighting {
        self.specular = Some(specular);
        self
    }

    pub fn roughness(mut self, roughness: f64) -> Lighting {
        self.roughness = Some(roughness);
        self
    }

    pub fn fresnel(mut self, fresnel: f64) -> Lighting {
        self.fresnel = Some(fresnel);
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
    pub fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct PlaneProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<bool>,
}

impl PlaneProject {
    pub fn new() -> PlaneProject {
        Default::default()
    }

    pub fn x(mut self, x: bool) -> PlaneProject {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: bool) -> PlaneProject {
        self.y = Some(y);
        self
    }

    pub fn z(mut self, z: bool) -> PlaneProject {
        self.z = Some(z);
        self
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct PlaneContours {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PlaneProject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "usecolormap")]
    use_colormap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    highlight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "highlightcolor")]
    highlight_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "highlightwidth")]
    highlight_width: Option<usize>,
}

impl PlaneContours {
    pub fn new() -> PlaneContours {
        Default::default()
    }

    pub fn show(mut self, show: bool) -> PlaneContours {
        self.show = Some(show);
        self
    }

    pub fn start(mut self, start: f64) -> PlaneContours {
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: f64) -> PlaneContours {
        self.end = Some(end);
        self
    }

    pub fn size(mut self, size: usize) -> PlaneContours {
        self.size = Some(size);
        self
    }

    pub fn project(mut self, project: PlaneProject) -> PlaneContours {
        self.project = Some(project);
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> PlaneContours {
        self.color = Some(color.to_color());
        self
    }

    pub fn use_colormap(mut self, use_colormap: bool) -> PlaneContours {
        self.use_colormap = Some(use_colormap);
        self
    }

    pub fn width(mut self, width: usize) -> PlaneContours {
        self.width = Some(width);
        self
    }

    pub fn highlight(mut self, highlight: bool) -> PlaneContours {
        self.highlight = Some(highlight);
        self
    }

    pub fn highlight_color<C: Color>(mut self, highlight_color: C) -> PlaneContours {
        self.highlight_color = Some(highlight_color.to_color());
        self
    }

    pub fn highlight_width(mut self, highlight_width: usize) -> PlaneContours {
        self.highlight_width = Some(highlight_width);
        self
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct SurfaceContours {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<PlaneContours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<PlaneContours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<PlaneContours>,
}

impl SurfaceContours {
    pub fn new() -> SurfaceContours {
        Default::default()
    }

    pub fn x(mut self, x: PlaneContours) -> SurfaceContours {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: PlaneContours) -> SurfaceContours {
        self.y = Some(y);
        self
    }

    pub fn z(mut self, z: PlaneContours) -> SurfaceContours {
        self.z = Some(z);
        self
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct Surface<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<Vec<X>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Vec<Y>>,
    z: Vec<Vec<Z>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<Visible>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "surfacecolor")]
    surface_color: Option<Vec<ColorWrapper>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cauto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmin: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmid: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectgaps")]
    connect_gaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contours: Option<SurfaceContours>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hidesurface")]
    hide_surface: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lighting: Option<Lighting>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lightposition")]
    light_position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zcalendar")]
    z_calendar: Option<Calendar>,
}

impl<X, Y, Z> Surface<X, Y, Z>
where
    X: Serialize + Default + Clone,
    Y: Serialize + Default + Clone,
    Z: Serialize + Default + Clone,
{
    pub fn new(z: Vec<Vec<Z>>) -> Box<Surface<X, Y, Z>> {
        Box::new(Surface {
            r#type: PlotType::Surface,
            z,
            ..Default::default()
        })
    }

    pub fn x(mut self, x: Vec<X>) -> Box<Surface<X, Y, Z>> {
        self.x = Some(x);
        Box::new(self)
    }

    pub fn y(mut self, y: Vec<Y>) -> Box<Surface<X, Y, Z>> {
        self.y = Some(y);
        Box::new(self)
    }

    pub fn name(mut self, name: &str) -> Box<Surface<X, Y, Z>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: Visible) -> Box<Surface<X, Y, Z>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Surface<X, Y, Z>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Surface<X, Y, Z>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Surface<X, Y, Z>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn surface_color<C: Color>(mut self, surface_color: Vec<C>) -> Box<Surface<X, Y, Z>> {
        let surface_color = private::to_color_array(surface_color);
        self.surface_color = Some(surface_color);
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Surface<X, Y, Z>> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Surface<X, Y, Z>> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Surface<X, Y, Z>> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Surface<X, Y, Z>> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Surface<X, Y, Z>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<Surface<X, Y, Z>> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(
        mut self,
        hover_template: Vec<S>,
    ) -> Box<Surface<X, Y, Z>> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn color_bar(mut self, color_bar: ColorBar) -> Box<Surface<X, Y, Z>> {
        self.color_bar = Some(color_bar);
        Box::new(self)
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> Box<Surface<X, Y, Z>> {
        self.auto_color_scale = Some(auto_color_scale);
        Box::new(self)
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> Box<Surface<X, Y, Z>> {
        self.color_scale = Some(color_scale);
        Box::new(self)
    }

    pub fn show_scale(mut self, show_scale: bool) -> Box<Surface<X, Y, Z>> {
        self.show_scale = Some(show_scale);
        Box::new(self)
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> Box<Surface<X, Y, Z>> {
        self.reverse_scale = Some(reverse_scale);
        Box::new(self)
    }

    pub fn cauto(mut self, cauto: bool) -> Box<Surface<X, Y, Z>> {
        self.cauto = Some(cauto);
        Box::new(self)
    }

    pub fn cmin(mut self, cmin: f64) -> Box<Surface<X, Y, Z>> {
        self.cmin = Some(cmin);
        Box::new(self)
    }

    pub fn cmax(mut self, cmax: f64) -> Box<Surface<X, Y, Z>> {
        self.cmax = Some(cmax);
        Box::new(self)
    }

    pub fn cmid(mut self, cmid: f64) -> Box<Surface<X, Y, Z>> {
        self.cmid = Some(cmid);
        Box::new(self)
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> Box<Surface<X, Y, Z>> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }

    pub fn contours(mut self, contours: SurfaceContours) -> Box<Surface<X, Y, Z>> {
        self.contours = Some(contours);
        Box::new(self)
    }

    pub fn hide_surface(mut self, hide_surface: bool) -> Box<Surface<X, Y, Z>> {
        self.hide_surface = Some(hide_surface);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Surface<X, Y, Z>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn lighting(mut self, lighting: Lighting) -> Box<Surface<X, Y, Z>> {
        self.lighting = Some(lighting);
        Box::new(self)
    }

    pub fn light_position(mut self, light_position: Position) -> Box<Surface<X, Y, Z>> {
        self.light_position = Some(light_position);
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Surface<X, Y, Z>> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Surface<X, Y, Z>> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }

    pub fn z_calendar(mut self, z_calendar: Calendar) -> Box<Surface<X, Y, Z>> {
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
        serde_json::to_string(&self).unwrap()
    }
}
